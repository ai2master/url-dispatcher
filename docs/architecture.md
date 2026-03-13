# 技术架构 | Technical Architecture

## 技术栈 | Tech Stack

| 组件 | 技术 | 用途 |
|------|------|------|
| 语言 | Rust 2021 Edition | 跨平台编译，无运行时依赖 |
| GUI | eframe / egui 0.30 | 即时模式 GUI，单二进制 |
| 剪贴板 | arboard 3 | 跨平台剪贴板访问 |
| 序列化 | serde + serde_json | 配置文件读写 |
| 路径 | dirs 6 | 平台相关的配置目录 |
| 注册表 | winreg 0.55 | Windows 注册表操作（条件编译） |
| CI/CD | GitHub Actions | 跨平台编译和发布 |

## 模块结构 | Module Structure

```
src/
├── main.rs          # 入口：命令行解析、模式选择、窗口初始化
│                    # Entry: CLI parsing, mode selection, window init
├── app.rs           # 应用状态（AppMode、Config）和 eframe::App 实现
│                    # App state and eframe::App implementation
├── config.rs        # 配置数据结构、JSON 加载/保存、默认值
│                    # Config types, JSON load/save, defaults
├── i18n.rs          # 国际化：中英文翻译、系统语言检测
│                    # i18n: Chinese/English translations, locale detection
├── actions.rs       # 动作执行：剪贴板复制、文件追加、浏览器启动
│                    # Action execution: clipboard, file append, browser launch
├── platform.rs      # 平台集成：Linux(.desktop/xdg-mime) / Windows(注册表)
│                    # Platform: Linux(.desktop/xdg-mime) / Windows(registry)
├── ui_dispatch.rs   # 分发弹窗 UI：URL 显示、动作按钮、键盘快捷键
│                    # Dispatch popup UI: URL display, action buttons, shortcuts
└── ui_settings.rs   # 设置 UI：动作管理、路径配置、系统注册
                     # Settings UI: action mgmt, path config, system registration
```

## 数据流 | Data Flow

```
操作系统调用（点击链接）
    │
    ▼
main.rs: 解析 args[1] 为 URL
    │
    ├── 有 URL → AppMode::Dispatch
    │       │
    │       ▼
    │   ui_dispatch.rs: 显示弹窗
    │       │
    │       ▼ 用户点击动作
    │   actions.rs: 执行动作
    │       ├── copy_to_clipboard()
    │       ├── append_to_file()
    │       └── open_in_browser()
    │
    └── 无 URL → AppMode::Settings
            │
            ▼
        ui_settings.rs: 设置界面
            ├── 动作 CRUD
            ├── config.rs: 保存配置
            └── platform.rs: 注册/取消注册
```

## 配置格式 | Config Format

```json
{
  "version": 1,
  "actions": [
    {
      "type": "CopyToClipboard",
      "id": "uuid-v4",
      "name": "复制到剪贴板",
      "enabled": true
    },
    {
      "type": "OpenInBrowser",
      "id": "uuid-v4",
      "name": "Firefox",
      "enabled": true,
      "executable_path": "/usr/bin/firefox",
      "args": ["{URL}"]
    }
  ],
  "append_file_path": "/home/user/urls.txt",
  "language": "Chinese"
}
```

## 平台注册机制 | Platform Registration

### Linux

1. 生成 `~/.local/share/applications/url-dispatcher.desktop`
2. 运行 `xdg-mime default url-dispatcher.desktop x-scheme-handler/http`
3. 运行 `xdg-mime default url-dispatcher.desktop x-scheme-handler/https`
4. 运行 `update-desktop-database`（可选）

### Windows

在 `HKEY_CURRENT_USER` 下写入：

1. `Software\Classes\URLDispatcherURL` — URL 协议处理
2. `Software\Classes\URLDispatcherURL\shell\open\command` — 启动命令
3. `Software\Clients\StartMenuInternet\URLDispatcher\Capabilities` — 应用能力
4. `Software\RegisteredApplications` — 注册到已注册应用列表

用户需在 Windows 设置中手动选择 URL Dispatcher 为默认浏览器。

## CI/CD 流程 | CI/CD Pipeline

```
push to main ──→ test job (ubuntu + windows)
                     └── cargo test

push tag v* ──→ test job
                  └── build job (ubuntu + windows)
                        └── release job
                              └── 创建 GitHub Release + 上传二进制
```

## 条件编译 | Conditional Compilation

- `#[cfg(target_os = "linux")]` — Linux 注册/取消注册
- `#[cfg(windows)]` — Windows 注册表操作、UI 提示
- `#[cfg(not(any(windows, target_os = "linux")))]` — 其他平台的回退提示
