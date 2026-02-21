# URL Dispatcher

[English](#english) | [中文](#中文)

---

## English

A cross-platform (Windows + Linux) URL dispatcher that can be registered as the default browser. When a URL is opened, it presents a popup with configurable actions instead of directly opening a browser.

### Features

- **Set as default browser** on Windows (registry) and Linux (xdg-mime / .desktop)
- **Configurable actions** via a graphical settings UI:
  - **Copy to Clipboard** — copy the URL
  - **Append to File** — append the URL (with timestamp) to a text file
  - **Open in Browser** — launch any browser with custom command-line arguments (e.g. `--incognito`, `--new-window`)
- **Keyboard shortcuts** — press 1–9 to quickly select an action, Esc to cancel
- **Bilingual UI** — supports English and Chinese, auto-detects system language
- **Portable config** — settings stored in a single JSON file

### UI Preview

When a URL is dispatched:

```
┌───────────────────────────────────────┐
│ URL Dispatcher                        │
├───────────────────────────────────────┤
│ URL:                                  │
│ https://example.com/some/page         │
│───────────────────────────────────────│
│ [1] Copy to Clipboard                 │
│ [2] Append to File                    │
│ [3] Open in Firefox                   │
│ [4] Open in Chrome (Incognito)        │
│───────────────────────────────────────│
│ [Settings]                   [Cancel] │
└───────────────────────────────────────┘
```

### Installation

#### Download

Pre-built binaries are available on the [Releases](https://github.com/aidev666888/url-dispatcher/releases) page:

- `url-dispatcher-linux-x86_64` — Linux (x86_64)
- `url-dispatcher-windows-x86_64.exe` — Windows (x86_64)

#### Build from Source

Requirements: [Rust](https://rustup.rs/) toolchain.

**Linux** (also needs system dependencies):

```bash
# Ubuntu/Debian
sudo apt-get install -y \
  libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev \
  libxkbcommon-dev libssl-dev libgtk-3-dev

cargo build --release
# Binary at: target/release/url-dispatcher
```

**Windows:**

```powershell
cargo build --release
# Binary at: target\release\url-dispatcher.exe
```

### Usage

#### Run Settings UI

Launch without arguments to open the settings window:

```bash
./url-dispatcher
```

From here you can:
- Add, edit, delete, and reorder actions
- Configure browser executables and command-line arguments
- Set the file path for "Append to File" actions
- Switch UI language (English / 中文)
- Register/unregister as the default browser

#### Register as Default Browser

Click **"Register as Default Browser"** in the settings UI.

**Linux:** Creates a `.desktop` file and runs `xdg-mime default` for `http` and `https` schemes. Takes effect immediately on most desktop environments.

**Windows:** Writes registry entries under `HKEY_CURRENT_USER`. After registering, go to **Settings > Apps > Default apps > Web browser** and select **URL Dispatcher**.

#### Dispatch a URL

Once registered as the default browser, clicking any link in another application will launch:

```bash
url-dispatcher "https://example.com"
```

A popup appears with your configured actions. Pick one, or press Esc to cancel.

### Configuration

Config file location:
- **Linux:** `~/.config/url-dispatcher/config.json`
- **Windows:** `%APPDATA%\URLDispatcher\config.json`

#### Example config

```json
{
  "version": 1,
  "actions": [
    {
      "type": "CopyToClipboard",
      "id": "...",
      "name": "Copy to Clipboard",
      "enabled": true
    },
    {
      "type": "OpenInBrowser",
      "id": "...",
      "name": "Chrome (Incognito)",
      "enabled": true,
      "executable_path": "/usr/bin/google-chrome",
      "args": ["--incognito", "{URL}"]
    }
  ],
  "append_file_path": "/home/user/urls.txt",
  "language": "English"
}
```

The `{URL}` placeholder in `args` is replaced with the actual URL at dispatch time.

---

## 中文

一个跨平台（Windows + Linux）的 URL 分发器，可以注册为系统默认浏览器。当打开一个 URL 时，会弹出一个窗口，显示可配置的操作选项，而不是直接用浏览器打开。

### 功能

- **注册为默认浏览器** — 支持 Windows（注册表）和 Linux（xdg-mime / .desktop 文件）
- **可配置的操作** — 通过图形设置界面管理：
  - **复制到剪贴板** — 复制 URL
  - **追加到文件** — 将 URL（带时间戳）追加到指定文本文件
  - **在浏览器中打开** — 用任意浏览器打开，可自定义命令行参数（如 `--incognito`、`--new-window`）
- **键盘快捷键** — 按 1–9 快速选择操作，Esc 取消
- **中英双语界面** — 自动检测系统语言，也可手动切换
- **便携配置** — 设置存储在单个 JSON 文件中

### 安装

#### 下载

预编译的二进制文件可在 [Releases](https://github.com/aidev666888/url-dispatcher/releases) 页面下载：

- `url-dispatcher-linux-x86_64` — Linux (x86_64)
- `url-dispatcher-windows-x86_64.exe` — Windows (x86_64)

#### 从源码编译

需要安装 [Rust](https://rustup.rs/) 工具链。

**Linux**（还需要系统依赖）：

```bash
# Ubuntu/Debian
sudo apt-get install -y \
  libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev \
  libxkbcommon-dev libssl-dev libgtk-3-dev

cargo build --release
# 二进制文件位于: target/release/url-dispatcher
```

**Windows:**

```powershell
cargo build --release
# 二进制文件位于: target\release\url-dispatcher.exe
```

### 使用方法

#### 打开设置界面

不带参数启动，打开设置窗口：

```bash
./url-dispatcher
```

在设置界面你可以：
- 添加、编辑、删除、排序操作
- 配置浏览器路径和命令行参数
- 设置"追加到文件"的文件路径
- 切换界面语言（English / 中文）
- 注册/取消注册为默认浏览器

#### 注册为默认浏览器

在设置界面点击 **"注册为默认浏览器"**。

**Linux:** 会创建 `.desktop` 文件并运行 `xdg-mime default` 注册 `http` 和 `https` 协议。在大多数桌面环境中立即生效。

**Windows:** 会在 `HKEY_CURRENT_USER` 下写入注册表项。注册后，需要前往 **设置 > 应用 > 默认应用 > Web 浏览器**，选择 **URL Dispatcher**。

#### 分发 URL

注册为默认浏览器后，在其他应用中点击任何链接都会启动：

```bash
url-dispatcher "https://example.com"
```

弹出窗口显示你配置的操作。选择一个，或按 Esc 取消。

### 配置文件

配置文件位置：
- **Linux:** `~/.config/url-dispatcher/config.json`
- **Windows:** `%APPDATA%\URLDispatcher\config.json`

`args` 中的 `{URL}` 占位符会在分发时被替换为实际的 URL。

## License / 许可证

MIT
