# 使用说明 | Usage Guide

## 安装 | Installation

### 下载预编译版本 | Download Pre-built Binaries

从 [Releases](https://github.com/ai2master/url-dispatcher/releases) 页面下载：

- `url-dispatcher-linux-x86_64` — Linux (x86_64)
- `url-dispatcher-windows-x86_64.exe` — Windows (x86_64)

Linux 下需要赋予执行权限：

```bash
chmod +x url-dispatcher-linux-x86_64
```

### 从源码编译 | Build from Source

需要 [Rust](https://rustup.rs/) 工具链。

**Linux（Ubuntu/Debian）：**

```bash
sudo apt-get install -y \
  libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev \
  libxkbcommon-dev libssl-dev libgtk-3-dev

cargo build --release
```

**Windows：**

```powershell
cargo build --release
```

编译产物位于 `target/release/` 目录。

---

## 基本使用 | Basic Usage

### 打开设置界面 | Open Settings UI

不带参数运行即可打开设置窗口：

```bash
./url-dispatcher
```

### URL 分发模式 | URL Dispatch Mode

带 URL 参数运行，弹出动作选择窗口：

```bash
./url-dispatcher "https://example.com"
```

---

## 注册为默认浏览器 | Register as Default Browser

### Linux

1. 打开设置界面
2. 点击 **"注册为默认浏览器"**
3. 立即生效（在大多数桌面环境中）

手动验证：

```bash
xdg-settings get default-web-browser
# 应输出: url-dispatcher.desktop
```

### Windows

1. 打开设置界面
2. 点击 **"注册为默认浏览器"**
3. 前往 **设置 > 应用 > 默认应用 > Web 浏览器**
4. 选择 **URL Dispatcher**

---

## 配置动作 | Configure Actions

### 添加动作 | Add Action

1. 在设置界面点击 **"+ 添加动作"**
2. 选择类型：
   - **复制到剪贴板** — 复制 URL
   - **追加到文件** — 将 URL（带时间戳）追加到文件
   - **在浏览器中打开** — 用指定浏览器打开
3. 填写名称和相关参数
4. 点击 **"保存"**

### 浏览器参数示例 | Browser Argument Examples

| 浏览器 | 可执行文件路径 | 参数 |
|--------|-------------|------|
| Firefox | `/usr/bin/firefox` | `{URL}` |
| Chrome 隐身 | `/usr/bin/google-chrome` | `--incognito {URL}` |
| Edge 新窗口 | `C:\Program Files\Microsoft\Edge\msedge.exe` | `--new-window {URL}` |
| Chrome Profile | `/usr/bin/google-chrome` | `--profile-directory="Profile 1" {URL}` |

`{URL}` 占位符会在分发时被替换为实际 URL。

### 编辑/删除/排序 | Edit/Delete/Reorder

- 每个动作右侧有 **编辑**、**删除**、**上移**、**下移** 按钮
- 复选框可以启用/禁用动作（不删除）
- 修改后点击 **"保存配置"**

---

## 键盘快捷键 | Keyboard Shortcuts

在分发模式下：

| 快捷键 | 功能 |
|--------|------|
| `1`-`9` | 选择对应序号的动作 |
| `Esc` | 取消并关闭窗口 |

---

## 语言切换 | Language Switch

设置界面右上角有语言下拉框，支持：
- English
- 中文

语言偏好会保存到配置文件中。

---

## 配置文件 | Configuration File

配置文件位置：

- **Linux:** `~/.config/url-dispatcher/config.json`
- **Windows:** `%APPDATA%\URLDispatcher\config.json`

可以直接编辑此 JSON 文件，也可以通过 GUI 管理。

---

## 取消注册 | Unregister

在设置界面点击 **"取消注册"** 即可恢复系统之前的默认浏览器。
