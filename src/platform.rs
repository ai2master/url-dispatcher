/*
 * 平台集成模块 | Platform Integration Module
 *
 * 功能概述 | Overview:
 * 本模块实现将 URL Dispatcher 注册/取消注册为系统默认浏览器的功能。
 * 由于不同操作系统的注册机制完全不同，本模块采用条件编译实现平台特定逻辑。
 *
 * This module implements functionality to register/unregister URL Dispatcher
 * as the system default browser. Since registration mechanisms vary completely
 * across operating systems, this module uses conditional compilation for
 * platform-specific logic.
 *
 * 平台支持 | Platform Support:
 * - Linux: 通过 .desktop 文件和 xdg-mime 命令注册
 *         Register via .desktop file and xdg-mime command
 * - Windows: 通过 HKCU 注册表键注册
 *           Register via HKCU registry keys
 * - macOS/其他: 暂不支持（返回错误）
 *              Not supported yet (returns error)
 *
 * 设计说明 | Design Notes:
 * - 使用条件编译 (#[cfg]) 为不同平台提供不同实现
 * - Linux 实现依赖 XDG 标准和 xdg-mime 工具
 * - Windows 实现使用 winreg crate 操作注册表
 * - 注册后仍需用户在系统设置中手动选择（特别是 Windows）
 *
 * - Uses conditional compilation (#[cfg]) for different platform implementations
 * - Linux implementation relies on XDG standards and xdg-mime tool
 * - Windows implementation uses winreg crate for registry operations
 * - User still needs to manually select in system settings after registration (especially Windows)
 */

use anyhow::{anyhow, Context, Result};
use std::path::Path;

// ═══════════════════════════════════════════════════════════════════════════
// Linux 平台实现 | Linux Platform Implementation
// ═══════════════════════════════════════════════════════════════════════════

/// Linux 平台：注册为默认浏览器 | Linux: Register as default browser
///
/// 在 Linux 上，通过创建符合 XDG 标准的 .desktop 文件并使用 xdg-mime
/// 命令将其设置为 http/https 协议的默认处理程序来实现注册。
///
/// On Linux, registration is achieved by creating an XDG-compliant .desktop file
/// and using xdg-mime command to set it as the default handler for http/https protocols.
///
/// 参数 | Parameters:
/// - `exe_path`: URL Dispatcher 可执行文件的完整路径
///              Full path to URL Dispatcher executable
///
/// .desktop 文件格式 | .desktop File Format:
/// - 遵循 FreeDesktop.org Desktop Entry Specification
///   Follows FreeDesktop.org Desktop Entry Specification
/// - 包含应用名称、描述、可执行路径、MIME 类型等元数据
///   Contains metadata like app name, description, executable path, MIME types
/// - %u 占位符表示传递单个 URL 参数
///   %u placeholder indicates passing a single URL argument
/// - 声明处理 x-scheme-handler/http 和 x-scheme-handler/https
///   Declares handling of x-scheme-handler/http and x-scheme-handler/https
///
/// 注册步骤 | Registration Steps:
/// 1. 确定本地应用目录（~/.local/share/applications/）
///    Determine local applications directory (~/.local/share/applications/)
/// 2. 创建 url-dispatcher.desktop 文件
///    Create url-dispatcher.desktop file
/// 3. 使用 xdg-mime 设置 http 协议默认处理程序
///    Use xdg-mime to set default handler for http protocol
/// 4. 使用 xdg-mime 设置 https 协议默认处理程序
///    Use xdg-mime to set default handler for https protocol
/// 5. 尝试更新桌面数据库（可选，某些发行版需要）
///    Try to update desktop database (optional, needed on some distros)
#[cfg(target_os = "linux")]
pub fn register_as_default_browser(exe_path: &Path) -> Result<()> {
    // 获取用户本地应用目录 | Get user local applications directory
    let apps_dir = dirs::data_local_dir()
        .ok_or_else(|| anyhow!("Cannot determine local data directory"))?
        .join("applications");
    std::fs::create_dir_all(&apps_dir)?;

    // 生成 .desktop 文件内容 | Generate .desktop file content
    let desktop_content = format!(
        r#"[Desktop Entry]
Version=1.0
Type=Application
Name=URL Dispatcher
Comment=Configurable URL dispatcher and browser selector
Exec={} %u
Terminal=false
Categories=Network;WebBrowser;
MimeType=x-scheme-handler/http;x-scheme-handler/https;
StartupNotify=true
"#,
        exe_path.display()
    );

    // 写入 .desktop 文件 | Write .desktop file
    let desktop_file = apps_dir.join("url-dispatcher.desktop");
    std::fs::write(&desktop_file, desktop_content)
        .context("Failed to write .desktop file")?;

    // 使用 xdg-mime 注册为 http 和 https 的默认处理程序
    // Use xdg-mime to register as default handler for http and https
    let status_http = std::process::Command::new("xdg-mime")
        .args(["default", "url-dispatcher.desktop", "x-scheme-handler/http"])
        .status();

    let status_https = std::process::Command::new("xdg-mime")
        .args(["default", "url-dispatcher.desktop", "x-scheme-handler/https"])
        .status();

    // 检查 xdg-mime 命令是否执行成功
    // Check if xdg-mime commands executed successfully
    if let Err(e) = status_http {
        return Err(anyhow!("Failed to run xdg-mime for http: {}", e));
    }
    if let Err(e) = status_https {
        return Err(anyhow!("Failed to run xdg-mime for https: {}", e));
    }

    // 更新桌面数据库（忽略错误，不一定所有系统都有这个命令）
    // Update desktop database (ignore errors, not all systems have this command)
    let _ = std::process::Command::new("update-desktop-database")
        .arg(&apps_dir)
        .status();

    Ok(())
}

/// Linux 平台：取消注册为默认浏览器 | Linux: Unregister as default browser
///
/// 删除 .desktop 文件并更新桌面数据库。注意：这只是移除应用的注册信息，
/// 不会自动将默认浏览器改回其他程序（用户需要手动设置）。
///
/// Removes .desktop file and updates desktop database. Note: This only removes
/// the application's registration, won't automatically change default browser
/// back to another program (user needs to set manually).
///
/// 清理逻辑 | Cleanup Logic:
/// 1. 定位 .desktop 文件
///    Locate .desktop file
/// 2. 如果文件存在，删除它
///    If file exists, delete it
/// 3. 尝试更新桌面数据库
///    Try to update desktop database
#[cfg(target_os = "linux")]
pub fn unregister_as_default_browser() -> Result<()> {
    // 获取应用目录 | Get applications directory
    let apps_dir = dirs::data_local_dir()
        .ok_or_else(|| anyhow!("Cannot determine local data directory"))?
        .join("applications");

    // 删除 .desktop 文件 | Delete .desktop file
    let desktop_file = apps_dir.join("url-dispatcher.desktop");
    if desktop_file.exists() {
        std::fs::remove_file(&desktop_file)?;
    }

    // 更新桌面数据库（忽略错误）| Update desktop database (ignore errors)
    let _ = std::process::Command::new("update-desktop-database")
        .arg(&apps_dir)
        .status();

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// Windows 平台实现 | Windows Platform Implementation
// ═══════════════════════════════════════════════════════════════════════════

/// Windows 平台：注册为默认浏览器 | Windows: Register as default browser
///
/// 在 Windows 上，通过在 HKEY_CURRENT_USER 注册表中创建一系列键值来实现注册。
/// 注册后，应用会出现在"设置 > 应用 > 默认应用 > Web 浏览器"的列表中，
/// 但仍需用户手动选择。
///
/// On Windows, registration is achieved by creating a series of keys and values
/// in HKEY_CURRENT_USER registry. After registration, the app will appear in
/// "Settings > Apps > Default apps > Web browser" list, but still requires
/// manual user selection.
///
/// 参数 | Parameters:
/// - `exe_path`: URL Dispatcher 可执行文件的完整路径
///              Full path to URL Dispatcher executable
///
/// 注册表键结构 | Registry Key Structure:
///
/// 1. Software\Classes\URLDispatcherURL
///    - 定义 URL 协议处理程序类
///    - Defines URL protocol handler class
///    - 包含 "URL Protocol" 空值标记
///    - Contains "URL Protocol" empty value marker
///    - shell\open\command: 指定启动命令，%1 为 URL 占位符
///    - shell\open\command: Specifies launch command, %1 is URL placeholder
///
/// 2. Software\Clients\StartMenuInternet\URLDispatcher
///    - 注册为可选的 Internet 客户端
///    - Registers as a selectable Internet client
///    - 在 Windows 设置中显示为可选浏览器
///    - Appears as selectable browser in Windows settings
///    - Capabilities: 声明应用能力和 URL 关联
///    - Capabilities: Declares app capabilities and URL associations
///
/// 3. Software\RegisteredApplications
///    - 将应用添加到已注册应用列表
///    - Adds app to registered applications list
///    - Windows 系统通过此列表识别可用浏览器
///    - Windows system identifies available browsers through this list
///
/// 关键步骤 | Key Steps:
/// 1. 创建 URLDispatcherURL 协议处理类及其命令
///    Create URLDispatcherURL protocol handler class and its command
/// 2. 注册为 StartMenuInternet 客户端
///    Register as StartMenuInternet client
/// 3. 声明应用能力（ApplicationName、ApplicationDescription）
///    Declare application capabilities (ApplicationName, ApplicationDescription)
/// 4. 设置 URL 关联（http、https）
///    Set URL associations (http, https)
/// 5. 添加到 RegisteredApplications
///    Add to RegisteredApplications
#[cfg(windows)]
pub fn register_as_default_browser(exe_path: &Path) -> Result<()> {
    use winreg::enums::*;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let exe_str = exe_path.display().to_string();

    // ─── 步骤 1: 创建 URL 协议处理类 | Step 1: Create URL protocol handler class ───
    let (class_key, _) = hkcu.create_subkey("Software\\Classes\\URLDispatcherURL")?;
    class_key.set_value("", &"URL Dispatcher")?;
    class_key.set_value("URL Protocol", &"")?;  // 空值表示这是 URL 协议 | Empty value indicates this is URL protocol

    // 设置协议处理命令，%1 会被 Windows 替换为实际 URL
    // Set protocol handler command, %1 will be replaced by Windows with actual URL
    let (cmd_key, _) =
        hkcu.create_subkey("Software\\Classes\\URLDispatcherURL\\shell\\open\\command")?;
    cmd_key.set_value("", &format!("\"{}\" \"%1\"", exe_str))?;

    // ─── 步骤 2: 注册为开始菜单 Internet 客户端 | Step 2: Register as StartMenuInternet client ───
    let (client_key, _) =
        hkcu.create_subkey("Software\\Clients\\StartMenuInternet\\URLDispatcher")?;
    client_key.set_value("", &"URL Dispatcher")?;

    // 设置客户端启动命令
    // Set client launch command
    let (client_cmd, _) =
        client_key.create_subkey("shell\\open\\command")?;
    client_cmd.set_value("", &format!("\"{}\"", exe_str))?;

    // ─── 步骤 3-4: 声明应用能力和 URL 关联 | Steps 3-4: Declare app capabilities and URL associations ───
    let (cap_key, _) = client_key.create_subkey("Capabilities")?;
    cap_key.set_value("ApplicationName", &"URL Dispatcher")?;
    cap_key.set_value(
        "ApplicationDescription",
        &"Configurable URL dispatcher and browser selector",
    )?;

    // 声明支持的 URL 协议 | Declare supported URL protocols
    let (urlassoc, _) = cap_key.create_subkey("URLAssociations")?;
    urlassoc.set_value("http", &"URLDispatcherURL")?;
    urlassoc.set_value("https", &"URLDispatcherURL")?;

    // ─── 步骤 5: 添加到已注册应用列表 | Step 5: Add to RegisteredApplications ───
    let (regapps, _) =
        hkcu.create_subkey("Software\\RegisteredApplications")?;
    regapps.set_value(
        "URLDispatcher",
        &"Software\\Clients\\StartMenuInternet\\URLDispatcher\\Capabilities",
    )?;

    Ok(())
}

/// Windows 平台：取消注册为默认浏览器 | Windows: Unregister as default browser
///
/// 删除所有在注册过程中创建的注册表键。注意：这只是移除注册信息，
/// 如果用户已将 URL Dispatcher 设为默认浏览器，需要手动选择其他浏览器。
///
/// Removes all registry keys created during registration. Note: This only
/// removes registration info. If user has set URL Dispatcher as default browser,
/// they need to manually select another browser.
///
/// 清理逻辑 | Cleanup Logic:
/// 1. 删除 URLDispatcherURL 协议处理类
///    Delete URLDispatcherURL protocol handler class
/// 2. 删除 StartMenuInternet 客户端条目
///    Delete StartMenuInternet client entry
/// 3. 从 RegisteredApplications 中移除条目
///    Remove entry from RegisteredApplications
#[cfg(windows)]
pub fn unregister_as_default_browser() -> Result<()> {
    use winreg::enums::*;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    // 删除协议处理类（忽略错误，可能不存在）
    // Delete protocol handler class (ignore errors, may not exist)
    let _ = hkcu.delete_subkey_all("Software\\Classes\\URLDispatcherURL");

    // 删除客户端条目（忽略错误，可能不存在）
    // Delete client entry (ignore errors, may not exist)
    let _ = hkcu.delete_subkey_all("Software\\Clients\\StartMenuInternet\\URLDispatcher");

    // 从已注册应用中移除（忽略错误，可能不存在）
    // Remove from registered applications (ignore errors, may not exist)
    if let Ok(regapps) =
        hkcu.open_subkey_with_flags("Software\\RegisteredApplications", KEY_WRITE)
    {
        let _ = regapps.delete_value("URLDispatcher");
    }

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// 其他平台回退实现 | Fallback Implementation for Other Platforms
// ═══════════════════════════════════════════════════════════════════════════

/// 其他平台（macOS 等）：注册功能占位符 | Other platforms (macOS etc.): Registration placeholder
///
/// 对于暂不支持的平台，返回友好的错误信息。
/// For platforms not yet supported, returns friendly error message.
#[cfg(not(any(windows, target_os = "linux")))]
pub fn register_as_default_browser(_exe_path: &Path) -> Result<()> {
    Err(anyhow!("Default browser registration is not supported on this platform"))
}

/// 其他平台（macOS 等）：取消注册功能占位符 | Other platforms (macOS etc.): Unregistration placeholder
///
/// 对于暂不支持的平台，返回友好的错误信息。
/// For platforms not yet supported, returns friendly error message.
#[cfg(not(any(windows, target_os = "linux")))]
pub fn unregister_as_default_browser() -> Result<()> {
    Err(anyhow!("Default browser unregistration is not supported on this platform"))
}
