# 可执行入口

此处预留 `traceweave.rs`（CLI）和 `traceweave-mcp.rs`（stdio MCP）入口。实现对应适配器时再创建文件；当前没有这两个可执行程序。

入口只负责启动、资源组装和退出状态，分别调用 `traceweave::adapters::cli`、`traceweave::adapters::mcp`。共享用例放在 `application`，不要在入口重复查询逻辑。

桌面入口位于 `frontend/src-tauri/src/main.rs`，不放在此处。
