# 桌面能力授权

需要前端调用 Tauri 系统能力时，在这里新增明确到窗口和操作的 capability 文件，并在 `tauri.conf.json` 中显式启用。

当前 `capabilities` 为空，无 capability JSON，不因目录存在就增加权限。自定义业务命令还需单独落实应用层授权，不能把 capability 配置当作数据库权限。
