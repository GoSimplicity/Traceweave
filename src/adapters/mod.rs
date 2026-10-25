//! 非桌面入口的参数、协议与输出适配；通过 application 调用共享用例。
//! Tauri 专属适配放在 frontend/src-tauri，避免核心依赖桌面运行库。

pub mod cli;
pub mod mcp;
