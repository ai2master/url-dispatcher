# URL Dispatcher

[![Build and Release](https://github.com/ai2master/url-dispatcher/actions/workflows/build.yml/badge.svg)](https://github.com/ai2master/url-dispatcher/actions/workflows/build.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20Windows-blue)](https://github.com/ai2master/url-dispatcher)

[中文](#中文) | [English](#english)

---

## 中文

### 一句话介绍

**URL Dispatcher** 是一个跨平台的智能 URL 分发器，可以注册为系统默认浏览器。当你点击任何链接时，它不会直接打开浏览器，而是弹出一个窗口让你选择如何处理这个 URL——复制到剪贴板、保存到文件、或用指定的浏览器（带自定义参数）打开。

A cross-platform intelligent URL dispatcher that can be registered as the system's default browser. Instead of directly opening a browser when you click any link, it presents a popup window letting you choose how to handle the URL—copy to clipboard, save to file, or open with a specified browser (with custom parameters).

---

### 功能特性

#### 1. 注册为默认浏览器
- **Windows 支持**：通过注册表（HKEY_CURRENT_USER）注册，在系统设置中显示为可选浏览器
- **Linux 支持**：通过 .desktop 文件和 xdg-mime 工具注册 http/https 协议处理程序
- **一键注册**：在设置界面点击按钮即可完成注册
- **一键取消**：支持完全清理注册信息

#### 2. 灵活的动作配置
- **复制到剪贴板**：一键复制 URL 到系统剪贴板，方便分享或保存
- **追加到文件**：将 URL（带时间戳）自动追加到指定文本文件，适合收集和整理链接
- **在浏览器中打开**：支持配置多个浏览器，每个浏览器可以有不同的启动参数
  - 支持任意命令行参数（如 `--incognito`、`--new-window`、`--profile-directory` 等）
  - 支持 `{URL}` 占位符，自动替换为实际 URL
  - 可配置多个同一浏览器的不同配置（如 Chrome 普通模式 + Chrome 隐身模式）

#### 3. 高效的键盘快捷键
- **数字键 1-9**：快速选择对应序号的动作
- **Esc 键**：取消并关闭分发窗口
- **鼠标点击**：支持鼠标点击操作

#### 4. 中英双语界面
- **自动检测**：根据系统语言环境（LANG、LC_ALL 等）自动选择界面语言
- **手动切换**：在设置界面可随时切换中文/英文
- **完整本地化**：所有界面文本、按钮、提示均已翻译

#### 5. 图形化设置管理
- **动作管理**：添加、编辑、删除、排序、启用/禁用动作
- **实时预览**：修改后立即在分发窗口中看到效果
- **配置保存**：所有设置保存在单个 JSON 文件中，可手动编辑
- **数据导出**：配置文件可备份、分享、版本控制

#### 6. 便携式部署
- **单文件可执行**：无需安装，下载即用
- **无运行时依赖**：不依赖 Python、Node.js、Java、.NET 等运行时
- **体积小巧**：编译后约 5-10 MB（已优化 strip + LTO）
- **启动快速**：Rust 编写，启动时间 < 100ms

---

### UI 预览

#### 分发模式（Dispatch Mode）

当你点击链接时弹出的窗口：

```
╔═══════════════════════════════════════════════╗
║ URL Dispatcher - https://example.com          ║
╠═══════════════════════════════════════════════╣
║                                               ║
║  URL:                                         ║
║  https://example.com/article/12345            ║
║                                               ║
╟───────────────────────────────────────────────╢
║                                               ║
║  [1] 复制到剪贴板                               ║
║  [2] 追加到文件                                 ║
║  [3] Firefox 默认                              ║
║  [4] Chrome 隐身模式                            ║
║  [5] Chrome 工作账号                            ║
║  [6] Edge 新窗口                               ║
║                                               ║
╟───────────────────────────────────────────────╢
║                                               ║
║  [设置]                              [取消]    ║
║                                               ║
╚═══════════════════════════════════════════════╝
```

#### 设置模式（Settings Mode）

不带参数运行时打开的设置界面：

```
╔═══════════════════════════════════════════════════════════╗
║ URL Dispatcher - 设置                                      ║
╠═══════════════════════════════════════════════════════════╣
║                                               语言: [中文▼] ║
║ ┌─────────────────────────────────────────────────────┐   ║
║ │ 动作列表                                             │   ║
║ │                                                     │   ║
║ │ ☑ [1] 复制到剪贴板          [编辑] [删除] [↑] [↓]    │   ║
║ │ ☑ [2] 追加到文件            [编辑] [删除] [↑] [↓]    │   ║
║ │ ☑ [3] Firefox              [编辑] [删除] [↑] [↓]    │   ║
║ │ ☑ [4] Chrome (隐身)         [编辑] [删除] [↑] [↓]    │   ║
║ │                                                     │   ║
║ │ [+ 添加动作]                                         │   ║
║ └─────────────────────────────────────────────────────┘   ║
║                                                           ║
║ 追加文件路径：                                             ║
║ /home/user/Documents/urls.txt                             ║
║                                                           ║
║ ──────────────────────────────────────────────────────    ║
║ 系统集成                                                   ║
║ [注册为默认浏览器]  [取消注册]                              ║
║ 提示：注册后在大多数桌面环境中立即生效                       ║
║                                                           ║
║ ──────────────────────────────────────────────────────    ║
║                                    [保存配置]              ║
╚═══════════════════════════════════════════════════════════╝
```

---

### 截图

*（TODO: 添加实际截图）*

- 分发弹窗截图
- 设置界面截图
- Windows 系统设置中的显示效果
- Linux 桌面环境中的显示效果

---

### 安装说明

#### 方式 1: 下载预编译版本（推荐）

从 [GitHub Releases](https://github.com/ai2master/url-dispatcher/releases) 页面下载最新版本：

**Linux (x86_64):**
```bash
# 下载
wget https://github.com/ai2master/url-dispatcher/releases/latest/download/url-dispatcher-linux-x86_64

# 赋予执行权限
chmod +x url-dispatcher-linux-x86_64

# 运行（可选：移动到 /usr/local/bin 以便全局访问）
sudo mv url-dispatcher-linux-x86_64 /usr/local/bin/url-dispatcher
```

**Windows (x86_64):**
1. 下载 `url-dispatcher-windows-x86_64.exe`
2. 将文件放到你喜欢的位置（如 `C:\Program Files\URLDispatcher\`）
3. 双击运行或通过命令行运行

#### 方式 2: 从源码编译

**前置要求:**
- [Rust](https://rustup.rs/) 工具链（1.70.0 或更高版本）

**Linux (Ubuntu/Debian) 依赖:**
```bash
# 安装编译依赖
sudo apt-get update
sudo apt-get install -y \
  libxcb-render0-dev \
  libxcb-shape0-dev \
  libxcb-xfixes0-dev \
  libxkbcommon-dev \
  libssl-dev \
  libgtk-3-dev \
  libatk1.0-dev \
  libglib2.0-dev \
  libpango1.0-dev

# 克隆仓库
git clone https://github.com/ai2master/url-dispatcher.git
cd url-dispatcher

# 编译 release 版本
cargo build --release

# 二进制文件位于: target/release/url-dispatcher
```

**Linux (Fedora/RHEL) 依赖:**
```bash
sudo dnf install -y \
  libxcb-devel \
  libxkbcommon-devel \
  openssl-devel \
  gtk3-devel \
  atk-devel \
  glib2-devel \
  pango-devel
```

**Linux (Arch) 依赖:**
```bash
sudo pacman -S libxcb libxkbcommon openssl gtk3
```

**Windows 依赖:**
```powershell
# Windows 不需要额外依赖，只需 Rust 工具链

# 克隆仓库
git clone https://github.com/ai2master/url-dispatcher.git
cd url-dispatcher

# 编译 release 版本
cargo build --release

# 二进制文件位于: target\release\url-dispatcher.exe
```

---

### 快速开始

#### 第 1 步：下载或编译

按照上面的安装说明获取可执行文件。

#### 第 2 步：配置动作

```bash
# 运行设置界面（不带参数）
./url-dispatcher
```

在设置界面中：
1. 点击 **"+ 添加动作"**
2. 选择动作类型并配置参数
3. 点击 **"保存配置"**

#### 第 3 步：注册为默认浏览器

在设置界面点击 **"注册为默认浏览器"**

**Linux**: 立即生效，可通过以下命令验证：
```bash
xdg-settings get default-web-browser
# 应输出: url-dispatcher.desktop
```

**Windows**: 注册后，需要手动设置：
1. 打开 **设置** → **应用** → **默认应用**
2. 点击 **Web 浏览器**
3. 选择 **URL Dispatcher**

现在，点击任何链接都会弹出 URL Dispatcher 的分发窗口！

---

### 详细使用说明

#### 配置"复制到剪贴板"动作

1. 打开设置界面
2. 点击 **"+ 添加动作"**
3. 类型选择：**复制到剪贴板**
4. 名称输入：`复制 URL`
5. 点击 **"保存"**

#### 配置"追加到文件"动作

1. 打开设置界面
2. 在 **"追加文件路径"** 输入框中输入文件路径，如：
   - Linux: `/home/username/Documents/urls.txt`
   - Windows: `C:\Users\username\Documents\urls.txt`
3. 点击 **"+ 添加动作"**
4. 类型选择：**追加到文件**
5. 名称输入：`保存到文件`
6. 点击 **"保存"**
7. 点击 **"保存配置"**

追加格式示例：
```
[2025-03-20 14:35:22] https://example.com/article1
[2025-03-20 14:36:10] https://github.com/ai2master/url-dispatcher
```

#### 配置"在浏览器中打开"动作

##### 常用浏览器路径和参数

| 浏览器 | Linux 路径 | Windows 路径 | 常用参数 |
|--------|-----------|-------------|---------|
| **Firefox** | `/usr/bin/firefox` | `C:\Program Files\Mozilla Firefox\firefox.exe` | `{URL}` (默认)<br>`-private-window {URL}` (隐私模式) |
| **Chrome** | `/usr/bin/google-chrome` | `C:\Program Files\Google\Chrome\Application\chrome.exe` | `{URL}` (默认)<br>`--incognito {URL}` (隐身模式)<br>`--new-window {URL}` (新窗口) |
| **Edge** | `/usr/bin/microsoft-edge` | `C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe` | `{URL}` (默认)<br>`--inprivate {URL}` (隐私模式) |
| **Brave** | `/usr/bin/brave-browser` | `C:\Program Files\BraveSoftware\Brave-Browser\Application\brave.exe` | `{URL}` (默认)<br>`--incognito {URL}` (隐身模式) |
| **Chromium** | `/usr/bin/chromium` 或 `/usr/bin/chromium-browser` | `C:\Program Files\Chromium\Application\chrome.exe` | `{URL}` (默认)<br>`--incognito {URL}` (隐身模式) |

##### 高级参数示例

1. **Chrome 特定配置文件**:
   ```
   --profile-directory="Profile 1" {URL}
   ```

2. **Chrome 禁用扩展**:
   ```
   --disable-extensions {URL}
   ```

3. **Firefox 特定配置文件**:
   ```
   -P "工作" {URL}
   ```

4. **组合多个参数**:
   ```
   --incognito --new-window {URL}
   ```

##### 添加浏览器动作的完整步骤

1. 打开设置界面
2. 点击 **"+ 添加动作"**
3. 类型选择：**在浏览器中打开**
4. 名称输入：`Chrome 隐身`
5. 可执行文件：
   - Linux: `/usr/bin/google-chrome`
   - Windows: `C:\Program Files\Google\Chrome\Application\chrome.exe`
6. 参数输入：`--incognito {URL}`
7. 点击 **"保存"**
8. 点击 **"保存配置"**

**注意**: 如果不确定浏览器路径，可以使用以下命令查找：

Linux:
```bash
which firefox
which google-chrome
which microsoft-edge
```

Windows (PowerShell):
```powershell
Get-Command firefox.exe | Select-Object Source
Get-Command chrome.exe | Select-Object Source
```

---

### 配置文件详解

配置文件位置：
- **Linux**: `~/.config/url-dispatcher/config.json`
- **Windows**: `%APPDATA%\URLDispatcher\config.json`

#### 完整配置示例

```json
{
  "version": 1,
  "actions": [
    {
      "type": "CopyToClipboard",
      "id": "550e8400-e29b-41d4-a716-446655440001",
      "name": "复制到剪贴板",
      "enabled": true
    },
    {
      "type": "AppendToFile",
      "id": "550e8400-e29b-41d4-a716-446655440002",
      "name": "追加到文件",
      "enabled": true
    },
    {
      "type": "OpenInBrowser",
      "id": "550e8400-e29b-41d4-a716-446655440003",
      "name": "Firefox 默认",
      "enabled": true,
      "executable_path": "/usr/bin/firefox",
      "args": ["{URL}"]
    },
    {
      "type": "OpenInBrowser",
      "id": "550e8400-e29b-41d4-a716-446655440004",
      "name": "Chrome 隐身",
      "enabled": true,
      "executable_path": "/usr/bin/google-chrome",
      "args": ["--incognito", "{URL}"]
    }
  ],
  "append_file_path": "/home/user/Documents/urls.txt",
  "language": "Chinese"
}
```

#### 字段说明

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `version` | 数字 | 是 | 配置格式版本，当前为 `1` |
| `actions` | 数组 | 是 | 动作列表，按数组顺序显示 |
| `append_file_path` | 字符串 | 否 | "追加到文件"动作的目标文件路径 |
| `language` | 字符串 | 否 | 界面语言，可选 `"English"` 或 `"Chinese"`，默认自动检测 |

#### Action 对象字段

##### 通用字段（所有类型）

| 字段 | 类型 | 说明 |
|------|------|------|
| `type` | 字符串 | 动作类型：`"CopyToClipboard"` / `"AppendToFile"` / `"OpenInBrowser"` |
| `id` | 字符串 | UUID v4 格式的唯一标识符 |
| `name` | 字符串 | 在界面中显示的动作名称 |
| `enabled` | 布尔值 | 是否启用此动作（禁用的动作不会在分发窗口显示） |

##### OpenInBrowser 特有字段

| 字段 | 类型 | 说明 |
|------|------|------|
| `executable_path` | 字符串 | 浏览器可执行文件的完整路径 |
| `args` | 字符串数组 | 命令行参数列表，支持 `{URL}` 占位符 |

#### 手动编辑配置文件

你可以直接编辑 JSON 文件而无需通过 GUI：

1. 用文本编辑器打开配置文件
2. 修改内容
3. 保存文件
4. 重新启动 URL Dispatcher（如果正在运行）

**提示**:
- 确保 JSON 格式正确（可使用 [JSONLint](https://jsonlint.com/) 验证）
- `id` 字段必须是有效的 UUID v4 格式
- 文件路径在 Windows 中使用 `\\` 或 `/`，不能使用单个 `\`

---

### 键盘快捷键

#### 分发窗口（Dispatch Window）

| 快捷键 | 功能 | 说明 |
|--------|------|------|
| `1` - `9` | 执行对应序号的动作 | 快速选择第 1-9 个动作 |
| `Esc` | 取消并关闭窗口 | 不执行任何动作 |

#### 设置窗口（Settings Window）

| 快捷键 | 功能 | 说明 |
|--------|------|------|
| `Alt + F4` (Windows) / `Ctrl + Q` (Linux) | 关闭窗口 | 系统标准关闭快捷键 |

---

### 常见问题 FAQ

#### 1. 注册后点击链接没有反应？

**Linux**:
- 检查注册状态：`xdg-settings get default-web-browser`
- 确认输出为 `url-dispatcher.desktop`
- 如果不是，重新在设置界面点击"注册为默认浏览器"
- 某些应用（如 Thunderbird）可能需要重启才能识别新的默认浏览器

**Windows**:
- 确认已在 **设置 > 应用 > 默认应用** 中手动选择了 URL Dispatcher
- 某些应用（如 Outlook）可能有自己的浏览器设置，需要单独配置

#### 2. 如何验证 URL Dispatcher 是否正确注册？

**Linux**:
```bash
xdg-settings get default-web-browser
```
应输出: `url-dispatcher.desktop`

**Windows**:
1. 打开 **设置** → **应用** → **默认应用**
2. 查看 **Web 浏览器** 是否显示为 **URL Dispatcher**

#### 3. "追加到文件"功能不工作？

检查以下几点：
- 确认在设置界面的"追加文件路径"中填写了有效路径
- 确认该路径的父目录存在且有写入权限
- 确认文件路径格式正确（Windows 使用 `C:\path\to\file.txt`，Linux 使用 `/path/to/file.txt`）
- 点击"保存配置"以保存路径设置

#### 4. 浏览器无法启动？

可能的原因：
- **可执行文件路径错误**: 检查浏览器路径是否正确，使用 `which` (Linux) 或 `where` (Windows) 命令查找
- **权限问题**: 确认可执行文件有执行权限
- **参数错误**: 检查命令行参数是否正确（尤其是引号和空格）
- **浏览器已卸载**: 确认浏览器仍然安装在指定位置

调试方法：
```bash
# Linux: 在终端中手动测试命令
/usr/bin/firefox "https://example.com"

# Windows: 在 PowerShell 中测试
& "C:\Program Files\Mozilla Firefox\firefox.exe" "https://example.com"
```

#### 5. 如何完全卸载 URL Dispatcher？

**Linux**:
```bash
# 1. 取消注册
./url-dispatcher  # 打开设置界面，点击"取消注册"

# 2. 删除配置文件
rm -rf ~/.config/url-dispatcher/

# 3. 删除 .desktop 文件（如果存在）
rm ~/.local/share/applications/url-dispatcher.desktop

# 4. 删除可执行文件
sudo rm /usr/local/bin/url-dispatcher
```

**Windows**:
```powershell
# 1. 取消注册（在设置界面点击"取消注册"）

# 2. 删除配置文件
Remove-Item -Recurse "$env:APPDATA\URLDispatcher"

# 3. 在"添加或删除程序"中将默认浏览器改回其他浏览器

# 4. 删除可执行文件
```

---

### 贡献指南

我们欢迎各种形式的贡献！

#### 报告 Bug

在 [GitHub Issues](https://github.com/ai2master/url-dispatcher/issues) 提交 bug 报告时，请包含：
- 操作系统和版本
- URL Dispatcher 版本
- 重现步骤
- 预期行为和实际行为
- 错误信息或日志

#### 提交功能建议

在 [GitHub Issues](https://github.com/ai2master/url-dispatcher/issues) 提交功能建议时，请说明：
- 功能的使用场景
- 预期的实现方式
- 是否愿意参与开发

#### 贡献代码

1. Fork 本仓库
2. 创建特性分支：`git checkout -b feature/your-feature`
3. 提交修改：`git commit -am 'Add some feature'`
4. 推送到分支：`git push origin feature/your-feature`
5. 创建 Pull Request

请确保：
- 代码遵循 Rust 标准风格（运行 `cargo fmt`）
- 通过所有测试（运行 `cargo test`）
- 添加了必要的注释和文档

---

### License / 许可证

MIT License - 详见 [LICENSE](LICENSE) 文件

---

### 仓库链接

GitHub: https://github.com/ai2master/url-dispatcher

---

## English

### One-Liner

**URL Dispatcher** is a cross-platform intelligent URL dispatcher that can be registered as your system's default browser. Instead of directly opening a browser when you click any link, it presents a popup window letting you choose how to handle the URL—copy to clipboard, save to file, or open with a specified browser (with custom parameters).

---

### Features

#### 1. Register as Default Browser
- **Windows Support**: Register via registry (HKEY_CURRENT_USER), appears as selectable browser in system settings
- **Linux Support**: Register via .desktop file and xdg-mime tool for http/https protocol handling
- **One-Click Registration**: Complete registration by clicking button in settings UI
- **One-Click Unregistration**: Fully clean up registration info

#### 2. Flexible Action Configuration
- **Copy to Clipboard**: One-click copy URL to system clipboard for easy sharing or saving
- **Append to File**: Automatically append URL (with timestamp) to specified text file, great for collecting and organizing links
- **Open in Browser**: Configure multiple browsers, each with different launch parameters
  - Supports arbitrary command-line arguments (e.g., `--incognito`, `--new-window`, `--profile-directory`)
  - Supports `{URL}` placeholder, automatically replaced with actual URL
  - Can configure multiple profiles of the same browser (e.g., Chrome normal + Chrome incognito)

#### 3. Efficient Keyboard Shortcuts
- **Number keys 1-9**: Quickly select corresponding action
- **Esc key**: Cancel and close dispatch window
- **Mouse click**: Supports mouse click operations

#### 4. Bilingual UI (Chinese/English)
- **Auto-detection**: Automatically selects UI language based on system locale (LANG, LC_ALL, etc.)
- **Manual switching**: Can switch between Chinese/English anytime in settings
- **Full localization**: All UI text, buttons, and prompts are translated

#### 5. Graphical Settings Management
- **Action management**: Add, edit, delete, reorder, enable/disable actions
- **Real-time preview**: See effects immediately in dispatch window after modifications
- **Config persistence**: All settings saved in single JSON file, can be manually edited
- **Data portability**: Config file can be backed up, shared, version controlled

#### 6. Portable Deployment
- **Single executable**: No installation needed, download and run
- **No runtime dependencies**: Doesn't depend on Python, Node.js, Java, .NET, etc.
- **Small size**: About 5-10 MB after compilation (optimized with strip + LTO)
- **Fast startup**: Written in Rust, startup time < 100ms

---

### UI Preview

#### Dispatch Mode

Popup when you click a link:

```
╔═══════════════════════════════════════════════╗
║ URL Dispatcher - https://example.com          ║
╠═══════════════════════════════════════════════╣
║                                               ║
║  URL:                                         ║
║  https://example.com/article/12345            ║
║                                               ║
╟───────────────────────────────────────────────╢
║                                               ║
║  [1] Copy to Clipboard                        ║
║  [2] Append to File                           ║
║  [3] Firefox Default                          ║
║  [4] Chrome Incognito                         ║
║  [5] Chrome Work Profile                      ║
║  [6] Edge New Window                          ║
║                                               ║
╟───────────────────────────────────────────────╢
║                                               ║
║  [Settings]                          [Cancel] ║
║                                               ║
╚═══════════════════════════════════════════════╝
```

#### Settings Mode

Settings UI when run without arguments:

```
╔═══════════════════════════════════════════════════════════╗
║ URL Dispatcher - Settings                                 ║
╠═══════════════════════════════════════════════════════════╣
║                                         Language: [En ▼]   ║
║ ┌─────────────────────────────────────────────────────┐   ║
║ │ Actions                                             │   ║
║ │                                                     │   ║
║ │ ☑ [1] Copy to Clipboard      [Edit] [Del] [↑] [↓]  │   ║
║ │ ☑ [2] Append to File         [Edit] [Del] [↑] [↓]  │   ║
║ │ ☑ [3] Firefox                [Edit] [Del] [↑] [↓]  │   ║
║ │ ☑ [4] Chrome (Incognito)     [Edit] [Del] [↑] [↓]  │   ║
║ │                                                     │   ║
║ │ [+ Add Action]                                      │   ║
║ └─────────────────────────────────────────────────────┘   ║
║                                                           ║
║ Append File Path:                                         ║
║ /home/user/Documents/urls.txt                             ║
║                                                           ║
║ ──────────────────────────────────────────────────────    ║
║ System Integration                                        ║
║ [Register as Default Browser]  [Unregister]               ║
║ Note: Takes effect immediately on most desktop envs       ║
║                                                           ║
║ ──────────────────────────────────────────────────────    ║
║                                    [Save Configuration]    ║
╚═══════════════════════════════════════════════════════════╝
```

---

### Screenshots

*(TODO: Add actual screenshots)*

- Dispatch popup screenshot
- Settings UI screenshot
- Display in Windows system settings
- Display in Linux desktop environment

---

### Installation

#### Method 1: Download Pre-built Binary (Recommended)

Download the latest version from [GitHub Releases](https://github.com/ai2master/url-dispatcher/releases):

**Linux (x86_64):**
```bash
# Download
wget https://github.com/ai2master/url-dispatcher/releases/latest/download/url-dispatcher-linux-x86_64

# Make executable
chmod +x url-dispatcher-linux-x86_64

# Run (optional: move to /usr/local/bin for global access)
sudo mv url-dispatcher-linux-x86_64 /usr/local/bin/url-dispatcher
```

**Windows (x86_64):**
1. Download `url-dispatcher-windows-x86_64.exe`
2. Place file in your preferred location (e.g., `C:\Program Files\URLDispatcher\`)
3. Double-click to run or run via command line

#### Method 2: Build from Source

**Prerequisites:**
- [Rust](https://rustup.rs/) toolchain (1.70.0 or higher)

**Linux (Ubuntu/Debian) Dependencies:**
```bash
# Install build dependencies
sudo apt-get update
sudo apt-get install -y \
  libxcb-render0-dev \
  libxcb-shape0-dev \
  libxcb-xfixes0-dev \
  libxkbcommon-dev \
  libssl-dev \
  libgtk-3-dev \
  libatk1.0-dev \
  libglib2.0-dev \
  libpango1.0-dev

# Clone repository
git clone https://github.com/ai2master/url-dispatcher.git
cd url-dispatcher

# Build release version
cargo build --release

# Binary located at: target/release/url-dispatcher
```

**Linux (Fedora/RHEL) Dependencies:**
```bash
sudo dnf install -y \
  libxcb-devel \
  libxkbcommon-devel \
  openssl-devel \
  gtk3-devel \
  atk-devel \
  glib2-devel \
  pango-devel
```

**Linux (Arch) Dependencies:**
```bash
sudo pacman -S libxcb libxkbcommon openssl gtk3
```

**Windows Dependencies:**
```powershell
# Windows requires no additional dependencies, just Rust toolchain

# Clone repository
git clone https://github.com/ai2master/url-dispatcher.git
cd url-dispatcher

# Build release version
cargo build --release

# Binary located at: target\release\url-dispatcher.exe
```

---

### Quick Start

#### Step 1: Download or Build

Follow installation instructions above to obtain the executable.

#### Step 2: Configure Actions

```bash
# Run settings UI (no arguments)
./url-dispatcher
```

In the settings UI:
1. Click **"+ Add Action"**
2. Select action type and configure parameters
3. Click **"Save Configuration"**

#### Step 3: Register as Default Browser

Click **"Register as Default Browser"** in settings UI

**Linux**: Takes effect immediately. Verify with:
```bash
xdg-settings get default-web-browser
# Should output: url-dispatcher.desktop
```

**Windows**: After registration, manually set:
1. Open **Settings** → **Apps** → **Default apps**
2. Click **Web browser**
3. Select **URL Dispatcher**

Now, clicking any link will show URL Dispatcher's dispatch window!

---

### Detailed Usage

#### Configure "Copy to Clipboard" Action

1. Open settings UI
2. Click **"+ Add Action"**
3. Type: Select **Copy to Clipboard**
4. Name: Enter `Copy URL`
5. Click **"Save"**

#### Configure "Append to File" Action

1. Open settings UI
2. In **"Append File Path"** field, enter file path, such as:
   - Linux: `/home/username/Documents/urls.txt`
   - Windows: `C:\Users\username\Documents\urls.txt`
3. Click **"+ Add Action"**
4. Type: Select **Append to File**
5. Name: Enter `Save to File`
6. Click **"Save"**
7. Click **"Save Configuration"**

Append format example:
```
[2025-03-20 14:35:22] https://example.com/article1
[2025-03-20 14:36:10] https://github.com/ai2master/url-dispatcher
```

#### Configure "Open in Browser" Action

##### Common Browser Paths and Arguments

| Browser | Linux Path | Windows Path | Common Arguments |
|---------|-----------|--------------|------------------|
| **Firefox** | `/usr/bin/firefox` | `C:\Program Files\Mozilla Firefox\firefox.exe` | `{URL}` (default)<br>`-private-window {URL}` (private) |
| **Chrome** | `/usr/bin/google-chrome` | `C:\Program Files\Google\Chrome\Application\chrome.exe` | `{URL}` (default)<br>`--incognito {URL}` (incognito)<br>`--new-window {URL}` (new window) |
| **Edge** | `/usr/bin/microsoft-edge` | `C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe` | `{URL}` (default)<br>`--inprivate {URL}` (private) |
| **Brave** | `/usr/bin/brave-browser` | `C:\Program Files\BraveSoftware\Brave-Browser\Application\brave.exe` | `{URL}` (default)<br>`--incognito {URL}` (incognito) |
| **Chromium** | `/usr/bin/chromium` or `/usr/bin/chromium-browser` | `C:\Program Files\Chromium\Application\chrome.exe` | `{URL}` (default)<br>`--incognito {URL}` (incognito) |

##### Advanced Argument Examples

1. **Chrome Specific Profile**:
   ```
   --profile-directory="Profile 1" {URL}
   ```

2. **Chrome Disable Extensions**:
   ```
   --disable-extensions {URL}
   ```

3. **Firefox Specific Profile**:
   ```
   -P "Work" {URL}
   ```

4. **Combine Multiple Arguments**:
   ```
   --incognito --new-window {URL}
   ```

##### Complete Steps to Add Browser Action

1. Open settings UI
2. Click **"+ Add Action"**
3. Type: Select **Open in Browser**
4. Name: Enter `Chrome Incognito`
5. Executable:
   - Linux: `/usr/bin/google-chrome`
   - Windows: `C:\Program Files\Google\Chrome\Application\chrome.exe`
6. Arguments: Enter `--incognito {URL}`
7. Click **"Save"**
8. Click **"Save Configuration"**

**Note**: If unsure about browser path, use these commands:

Linux:
```bash
which firefox
which google-chrome
which microsoft-edge
```

Windows (PowerShell):
```powershell
Get-Command firefox.exe | Select-Object Source
Get-Command chrome.exe | Select-Object Source
```

---

### Configuration File Explained

Config file location:
- **Linux**: `~/.config/url-dispatcher/config.json`
- **Windows**: `%APPDATA%\URLDispatcher\config.json`

#### Complete Configuration Example

```json
{
  "version": 1,
  "actions": [
    {
      "type": "CopyToClipboard",
      "id": "550e8400-e29b-41d4-a716-446655440001",
      "name": "Copy to Clipboard",
      "enabled": true
    },
    {
      "type": "AppendToFile",
      "id": "550e8400-e29b-41d4-a716-446655440002",
      "name": "Append to File",
      "enabled": true
    },
    {
      "type": "OpenInBrowser",
      "id": "550e8400-e29b-41d4-a716-446655440003",
      "name": "Firefox Default",
      "enabled": true,
      "executable_path": "/usr/bin/firefox",
      "args": ["{URL}"]
    },
    {
      "type": "OpenInBrowser",
      "id": "550e8400-e29b-41d4-a716-446655440004",
      "name": "Chrome Incognito",
      "enabled": true,
      "executable_path": "/usr/bin/google-chrome",
      "args": ["--incognito", "{URL}"]
    }
  ],
  "append_file_path": "/home/user/Documents/urls.txt",
  "language": "English"
}
```

#### Field Descriptions

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `version` | Number | Yes | Config format version, currently `1` |
| `actions` | Array | Yes | Action list, displayed in array order |
| `append_file_path` | String | No | Target file path for "Append to File" action |
| `language` | String | No | UI language, either `"English"` or `"Chinese"`, auto-detected by default |

#### Action Object Fields

##### Common Fields (All Types)

| Field | Type | Description |
|-------|------|-------------|
| `type` | String | Action type: `"CopyToClipboard"` / `"AppendToFile"` / `"OpenInBrowser"` |
| `id` | String | Unique identifier in UUID v4 format |
| `name` | String | Action name displayed in UI |
| `enabled` | Boolean | Whether this action is enabled (disabled actions won't show in dispatch window) |

##### OpenInBrowser Specific Fields

| Field | Type | Description |
|-------|------|-------------|
| `executable_path` | String | Full path to browser executable |
| `args` | String Array | Command-line argument list, supports `{URL}` placeholder |

#### Manually Edit Configuration File

You can edit the JSON file directly without using the GUI:

1. Open config file with text editor
2. Modify content
3. Save file
4. Restart URL Dispatcher (if running)

**Tips**:
- Ensure JSON format is correct (validate with [JSONLint](https://jsonlint.com/))
- `id` field must be valid UUID v4 format
- File paths in Windows use `\\` or `/`, cannot use single `\`

---

### Keyboard Shortcuts

#### Dispatch Window

| Shortcut | Function | Description |
|----------|----------|-------------|
| `1` - `9` | Execute corresponding action | Quickly select actions 1-9 |
| `Esc` | Cancel and close window | Don't execute any action |

#### Settings Window

| Shortcut | Function | Description |
|----------|----------|-------------|
| `Alt + F4` (Windows) / `Ctrl + Q` (Linux) | Close window | System standard close shortcut |

---

### FAQ (Frequently Asked Questions)

#### 1. Clicks on links have no response after registration?

**Linux**:
- Check registration status: `xdg-settings get default-web-browser`
- Confirm output is `url-dispatcher.desktop`
- If not, click "Register as Default Browser" again in settings
- Some apps (like Thunderbird) may need restart to recognize new default browser

**Windows**:
- Confirm you've manually selected URL Dispatcher in **Settings > Apps > Default apps**
- Some apps (like Outlook) may have their own browser settings, need separate configuration

#### 2. How to verify URL Dispatcher is correctly registered?

**Linux**:
```bash
xdg-settings get default-web-browser
```
Should output: `url-dispatcher.desktop`

**Windows**:
1. Open **Settings** → **Apps** → **Default apps**
2. Check if **Web browser** shows **URL Dispatcher**

#### 3. "Append to File" feature not working?

Check these points:
- Confirm valid path filled in "Append File Path" in settings UI
- Confirm parent directory of path exists and has write permission
- Confirm file path format is correct (Windows uses `C:\path\to\file.txt`, Linux uses `/path/to/file.txt`)
- Click "Save Configuration" to save path setting

#### 4. Browser fails to launch?

Possible causes:
- **Incorrect executable path**: Check browser path is correct, use `which` (Linux) or `where` (Windows) to find
- **Permission issues**: Confirm executable has execute permission
- **Incorrect arguments**: Check command-line arguments are correct (especially quotes and spaces)
- **Browser uninstalled**: Confirm browser still installed at specified location

Debug method:
```bash
# Linux: manually test command in terminal
/usr/bin/firefox "https://example.com"

# Windows: test in PowerShell
& "C:\Program Files\Mozilla Firefox\firefox.exe" "https://example.com"
```

#### 5. How to completely uninstall URL Dispatcher?

**Linux**:
```bash
# 1. Unregister
./url-dispatcher  # Open settings UI, click "Unregister"

# 2. Delete config files
rm -rf ~/.config/url-dispatcher/

# 3. Delete .desktop file (if exists)
rm ~/.local/share/applications/url-dispatcher.desktop

# 4. Delete executable
sudo rm /usr/local/bin/url-dispatcher
```

**Windows**:
```powershell
# 1. Unregister (click "Unregister" in settings UI)

# 2. Delete config files
Remove-Item -Recurse "$env:APPDATA\URLDispatcher"

# 3. Change default browser back to another browser in "Apps and Features"

# 4. Delete executable
```

---

### Contributing

We welcome all kinds of contributions!

#### Report Bugs

When submitting bug reports on [GitHub Issues](https://github.com/ai2master/url-dispatcher/issues), please include:
- OS and version
- URL Dispatcher version
- Steps to reproduce
- Expected vs actual behavior
- Error messages or logs

#### Submit Feature Requests

When submitting feature requests on [GitHub Issues](https://github.com/ai2master/url-dispatcher/issues), please explain:
- Use case for the feature
- Expected implementation approach
- Whether you're willing to participate in development

#### Contribute Code

1. Fork this repository
2. Create feature branch: `git checkout -b feature/your-feature`
3. Commit changes: `git commit -am 'Add some feature'`
4. Push to branch: `git push origin feature/your-feature`
5. Create Pull Request

Please ensure:
- Code follows Rust standard style (run `cargo fmt`)
- All tests pass (run `cargo test`)
- Added necessary comments and documentation

---

### License

MIT License - See [LICENSE](LICENSE) file for details

---

### Repository

GitHub: https://github.com/ai2master/url-dispatcher
