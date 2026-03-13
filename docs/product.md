# 产品介绍 | Product Introduction

## URL Dispatcher — 智能 URL 分发器

### 一句话介绍 | One-liner

一个跨平台的默认浏览器替代工具，让你在打开每个链接时都能选择如何处理。

A cross-platform default browser replacement that lets you choose how to handle every link you open.

---

## 核心场景 | Key Use Cases

### 1. 多浏览器用户

你同时使用 Chrome（工作）和 Firefox（个人），每次点链接都想选择用哪个打开。

### 2. URL 收集者

你需要把各种链接保存到一个文件里，方便后续整理。URL Dispatcher 可以一键追加到文本文件，自动带时间戳。

### 3. 隐私敏感用户

某些链接你想用隐身模式打开，某些想用普通模式。配置不同的浏览器参数即可。

### 4. 开发者

需要在不同浏览器（Chrome、Firefox、Edge）中测试链接的开发者。

---

## 功能亮点 | Feature Highlights

| 功能 | 说明 |
|------|------|
| 注册为默认浏览器 | 支持 Windows + Linux，一键注册 |
| 可配置动作 | 复制 URL、追加到文件、用指定浏览器打开 |
| 自定义浏览器参数 | 支持 `--incognito`、`--new-window` 等任意参数 |
| 键盘快捷键 | 1-9 快速选择，Esc 取消 |
| 中英双语界面 | 自动检测系统语言，手动可切换 |
| 单文件部署 | 无运行时依赖，单个可执行文件 |
| 开源免费 | MIT 许可证 |

---

## 与同类工具对比 | Comparison

| 特性 | URL Dispatcher | [Browserosaurus](https://browserosaurus.com/) | [BrowserSelect](https://github.com/nicjhan/browserselect) |
|------|---------------|----------------------------------------------|-----------------------------------------------------------|
| 平台 | Windows + Linux | macOS only | Windows only |
| 复制 URL | 支持 | 支持 | 不支持 |
| 追加到文件 | 支持 | 不支持 | 不支持 |
| 自定义参数 | 支持 | 不支持 | 部分支持 |
| 中文界面 | 支持 | 不支持 | 不支持 |
| 运行时依赖 | 无 | Electron | .NET |
| 开源 | MIT | MIT | MIT |

---

## 技术优势 | Technical Advantages

- **Rust 编写**：内存安全、无 GC 停顿、启动极快（< 100ms）
- **单二进制**：没有 Python/Node.js/Java 等运行时依赖
- **体积小**：release 版本约 5-10 MB（strip + LTO 优化）
- **跨平台编译**：GitHub Actions 自动构建 Windows 和 Linux 版本

---

## 路线图 | Roadmap

- [ ] macOS 支持
- [ ] URL 规则匹配（根据域名自动选择浏览器）
- [ ] URL 历史记录
- [ ] 系统托盘常驻模式
- [ ] 更多语言支持
