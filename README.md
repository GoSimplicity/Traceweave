# Traceweave · 知径

Reusable business queries for humans and AI.

Traceweave 是一个处于设计阶段的业务查询工作台，面向开发者、技术支持、业务分析人员，以及通过 MCP 接入的 AI 助手。它希望把一次查询中用到的步骤、参数关系和业务口径保存下来，让下一次换个订单、用户或时间范围时，无需从头找表、复制 ID、修改 SQL。

> **当前状态：基础骨架。** 已创建 Rust 核心空库、Vue＋TypeScript 空页面和 Tauri 2 桌面入口。桌面壳用于加载前端，没有业务逻辑、IPC 命令、数据库连接、安装包或 MCP 服务。下文描述的是产品设计与开发计划。

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

## 本地开发

Rust 核心使用 Rust 2024 edition，无第三方依赖。根目录 `Cargo.toml` 同时定义核心 package 和 workspace；`src/lib.rs` 是共享核心入口，目前只有模块说明。`frontend/src-tauri/` 是依赖核心的薄桌面 package，单独持有 Tauri 依赖。

在仓库根目录检查骨架：

```sh
cargo check --locked --offline
cargo fmt --check
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
