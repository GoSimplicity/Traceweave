# Traceweave · 知径

Reusable business queries for humans and AI.

Traceweave 是一个处于设计阶段的业务查询工作台，面向开发者、技术支持、业务分析人员，以及通过 MCP 接入的 AI 助手。它希望把一次查询中用到的步骤、参数关系和业务口径保存下来，让下一次换个订单、用户或时间范围时，无需从头找表、复制 ID、修改 SQL。

> **当前状态：基础骨架。** 已建立 Rust 核心模块、前端功能目录、Tauri 2 桌面入口，以及测试和迁移目录。新增模块只有声明和职责说明；桌面仍显示空页面，没有业务逻辑、IPC 命令、数据库连接、安装包或 MCP 服务。下文产品能力仍是开发计划。

## 从一次探索到重复执行

例如，排查一笔订单对应的用户收益：

1. 输入订单号，查询订单并取得 `user_id`。
2. 根据 `user_id` 查询用户信息和指定时间段的收益。
3. 确认有效步骤，标出订单号、时间范围等输入，补充收益计算口径，保存为查询链路。
4. 下次只更换订单号或时间范围，按已确认的链路重新查询。

AI 助手也应能在新会话中找到这条链路，读取输入要求与业务说明后执行，而不必重新理解整个数据库、重写同一组查询。

保存的核心是查询方法和业务口径。再次执行默认读取当前数据；SQL 执行成功，也不等于业务口径已经正确。

## 首版计划

| 能力 | 预期行为 |
| --- | --- |
| 范围初始化 | 选择数据源、schema 和表，采集所选范围内的结构；不默认读取业务样本 |
| 查询探索 | 执行只读查询，显式记录参数来自用户输入还是上游结果 |
| 链路复用 | 将有效步骤整理成草稿，经用户确认后保存版本并重复执行 |
| 时间窗口 | 支持指定年月、上周和自定义范围，执行前明确日期、时区与边界 |
| 人与 AI 共用 | 工作台与 MCP 复用同一套查询执行、权限检查和业务规则 |
| 结果追溯 | 返回链路版本、实际条件、来源与完整性状态，区分空结果、失败和截断 |

首版只读访问业务数据库。界面中选择范围不能替代数据库自身的权限控制；AI 也不能自行扩大范围或确认自己生成的业务口径。

首版暂不包含写库、任意脚本、复杂分支循环、批量调度、云同步和团队权限管理。计划由外部 AI 助手接入，不内置模型。

## 技术方向与开发顺序

核心使用 Rust，从一个 Cargo package 开始，让 CLI、stdio MCP 和后续工作台复用应用逻辑；有实际依赖隔离需求时再拆分 crate。

拟采用 PostgreSQL 作为首版业务数据库，SQLite 保存本地元数据，尚未引入数据库依赖。前端骨架使用 Vue 3、TypeScript 与 Vite，桌面壳使用 Tauri 2；完整工作台能力仍待技术样机验证。

开发将按以下顺序推进：

1. 使用隔离数据库和合成数据，验证动态查询类型、只读权限边界、参数绑定及取消行为。
2. 实现核心、CLI 和 MCP，跑通“首次探索 → 保存链路 → 更换参数 → 新会话复用”。
3. 完成人类工作台，让人和 AI 都能完成同一条查询流程。

## 代码目录与开发边界

```text
Traceweave/
├── Cargo.toml                 # 核心 package + 两个 package 的 workspace
├── Cargo.lock                 # 统一 Rust 依赖锁文件
├── src/
│   ├── lib.rs                 # 共享核心模块入口
│   ├── application/           # 共用用例入口与输入输出契约
│   ├── workspace/             # 数据源引用、授权范围、时间约定
│   ├── catalog/               # 结构目录、采集覆盖状态、差异
│   ├── exploration/           # 探索会话、查询草稿、显式绑定
│   ├── chain/                 # 链路、参数、依赖、时间窗口、版本
│   ├── executor/              # 调度、结果、取消、重跑
│   ├── policy/                # 范围、只读规则、输出与资源限制
│   ├── postgres/              # 业务库连接、元数据读取、参数化查询
│   ├── store/                 # SQLite 元数据、历史、迁移
│   ├── credentials/           # 系统凭据与会话凭据生命周期
│   ├── adapters/
│   │   ├── cli/               # 命令行输入输出适配
│   │   └── mcp/               # MCP 协议与工具适配
│   └── bin/                   # CLI/MCP 可执行入口预留，当前只有说明
├── migrations/                # 仅本地 SQLite，当前没有 SQL
├── tests/
│   └── fixtures/              # 核心集成测试与合成数据预留
└── frontend/
    ├── package.json           # 前端及桌面开发命令
    ├── index.html             # WebView/浏览器页面入口
    ├── src/
    │   ├── main.ts            # Vue 启动
    │   ├── App.vue            # 当前空白根组件
    │   ├── app/               # 布局、导航、跨功能页面组装
    │   ├── features/
    │   │   ├── workspace/     # 工作区、初始化、范围选择
    │   │   ├── catalog/       # 结构浏览与采集状态
    │   │   ├── exploration/   # SQL 探索、步骤与绑定
    │   │   ├── chain/         # 链路检索、编辑与版本确认
    │   │   └── runs/          # 当前执行、结果、历史与重跑
    │   ├── components/        # 跨功能展示组件
    │   ├── composables/       # 跨功能响应式逻辑
    │   ├── lib/ipc/           # 桌面调用、事件和传输类型边界
    │   ├── styles/            # 全局样式与设计变量
    │   └── assets/            # 参与前端构建的静态资源
    ├── src-tauri/
    │   ├── Cargo.toml         # 薄桌面 package，依赖根核心
    │   ├── build.rs           # Tauri 构建准备
    │   ├── tauri.conf.json    # 窗口、资源与安全配置
    │   ├── capabilities/     # 系统能力授权预留，当前无授权文件
    │   ├── icons/             # 桌面占位图标
    │   └── src/
    │       ├── main.rs        # 桌面启动入口
    │       ├── commands/      # IPC 命令适配，当前无注册命令
    │       └── state/         # 桌面进程资源与服务句柄
    └── tests/
        └── e2e/              # 跨功能与端到端测试预留
```

Rust 目录使用 `mod.rs` 作为模块入口，已由 `lib.rs` 或桌面 `main.rs` 声明，因此会参与编译检查。前端、测试和迁移预留目录用简短 README 保存职责，尚未接入运行流程；不通过空函数、假数据或未实现接口占位。

后续代码按以下边界落位：

- **调用方向**：Vue 功能 → `lib/ipc` → 桌面 `commands` → 核心 `application`；CLI/MCP 适配器也调用 `application`，由它协调核心模块。这是待落实的开发约定，当前没有整条调用链。
- **核心出口**：只公开 `application` 和 `adapters` 模块，业务模块先保持库内私有。新增跨入口契约从应用层定义或显式重导出，不让桌面直接调用数据库模块。
- **状态归属**：业务范围、链路版本、执行状态以 Rust 为准；前端管理输入草稿、选择、加载和展示状态。数据库连接与系统凭据由 Rust 持有。
- **功能归属**：页面、局部组件、局部 composable 和测试先放对应 `features` 目录。`runs` 同时承载当前执行和历史；不同功能间只做明确的单向复用，避免循环引用。有实际共用代码时再提取到公共目录。
- **新增文件**：例如结构采集的规则放 `catalog`，实际 PG 访问放 `postgres`，结构树组件放 `features/catalog`。每个模块按实际代码增长拆文件，不统一铺设空的 controller/service/repository 层。
- **测试与迁移**：模块单测就近放置，跨模块及数据库集成测试放根 `tests`，界面完整流程放 `frontend/tests/e2e`。`migrations` 只服务 SQLite；业务 PG 的隔离测试样本放 `tests/fixtures`。

当前没有 CLI/MCP 可执行文件、路由、全局状态库、数据库驱动或测试框架。实现第一个相关用例时再引入所需依赖和真实测试。目录边界依据既定产品范围划分，不意味着其中能力已经实现。

目录约定参考 [Cargo 项目布局](https://doc.rust-lang.org/cargo/guide/project-layout.html)、[Rust 模块文件](https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html)和 [Tauri 前后端调用](https://v2.tauri.app/develop/calling-rust/)；前端按产品功能分组是本项目的组织选择。

## 本地开发

Rust 核心使用 Rust 2024 edition，无第三方依赖。根目录 `Cargo.toml` 同时定义核心 package 和 workspace；`src/lib.rs` 是共享核心入口，目前只有模块声明和职责说明。`frontend/src-tauri/` 是依赖核心的薄桌面 package，单独持有 Tauri 依赖。

在仓库根目录检查骨架：

```sh
cargo check --locked --offline
cargo fmt --all --check
```

Rust 骨架验证环境为 Rust/Cargo 1.96.0。根目录的默认 Cargo 命令仅检查核心；桌面依赖最低要求 Rust 1.90。当前没有业务测试。

前端位于 `frontend/`，使用 npm 管理依赖。Node.js 需满足 Vite 的要求：20.19+（20.x）或 22.12+；建议使用受支持的 LTS 版本。

```sh
cd frontend
npm ci
npm run dev
```

打开终端打印的本地地址，当前应显示空白页面。入口关系为 `index.html` → `src/main.ts` → `src/App.vue`，尚未连接 Rust。运行 `npm run build` 会先检查 TypeScript/Vue 类型，再生成 `dist/`。依赖精确版本记录在 `package.json` 和 `package-lock.json` 中。

### 桌面空窗口

本步验证目标为 macOS，需安装 Xcode Command Line Tools 或完整 Xcode。其他平台的系统依赖见 [Tauri 官方前置条件](https://v2.tauri.app/start/prerequisites/)。

在 `frontend/` 目录运行：

```sh
npm run tauri -- dev
```

该命令自动启动 Vite，再编译并打开桌面窗口；无需另外运行 `npm run dev`。开发服务器固定使用 `127.0.0.1:5173`，端口占用时会报错，不自动换端口。当前窗口内容为空，没有向前端开放系统操作权限或业务命令。

构建嵌入前端资源的本地调试程序（不生成安装包）：

```sh
npm run tauri -- build --debug --no-bundle
```

macOS 可执行文件位于根目录 `target/debug/traceweave-desktop`。应用标识 `local.traceweave.desktop` 仅用于当前本地开发；使用满足 Tauri 编译要求的纯色占位 PNG，签名、分发标识和正式图标尚未配置。Rust 依赖统一记录在根目录 `Cargo.lock`，Tauri 生成的 `gen/` 与构建产物不纳入 Git。

详细设计文档和本地开发约定不随仓库分发。

## 许可证

[Apache License 2.0](LICENSE)
