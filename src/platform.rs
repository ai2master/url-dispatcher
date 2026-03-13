// 平台集成：注册/取消注册为默认浏览器 | Platform integration: register/unregister as default browser

use anyhow::{anyhow, Context, Result};
use std::path::Path;

// ─── Linux：通过 .desktop 文件和 xdg-mime 注册 | Linux: register via .desktop file and xdg-mime ───

#[cfg(target_os = "linux")]
pub fn register_as_default_browser(exe_path: &Path) -> Result<()> {
    let apps_dir = dirs::data_local_dir()
        .ok_or_else(|| anyhow!("Cannot determine local data directory"))?
        .join("applications");
    std::fs::create_dir_all(&apps_dir)?;

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

    let desktop_file = apps_dir.join("url-dispatcher.desktop");
    std::fs::write(&desktop_file, desktop_content)
        .context("Failed to write .desktop file")?;

    // 注册为 http 和 https 的默认处理程序 | Register as default handler for http and https
    let status_http = std::process::Command::new("xdg-mime")
        .args(["default", "url-dispatcher.desktop", "x-scheme-handler/http"])
        .status();

    let status_https = std::process::Command::new("xdg-mime")
        .args(["default", "url-dispatcher.desktop", "x-scheme-handler/https"])
        .status();

    if let Err(e) = status_http {
        return Err(anyhow!("Failed to run xdg-mime for http: {}", e));
    }
    if let Err(e) = status_https {
        return Err(anyhow!("Failed to run xdg-mime for https: {}", e));
    }

    // 更新桌面数据库（忽略错误，不一定可用）| Update desktop database (ignore errors, not always available)
    let _ = std::process::Command::new("update-desktop-database")
        .arg(&apps_dir)
        .status();

    Ok(())
}

#[cfg(target_os = "linux")]
pub fn unregister_as_default_browser() -> Result<()> {
    let apps_dir = dirs::data_local_dir()
        .ok_or_else(|| anyhow!("Cannot determine local data directory"))?
        .join("applications");

    let desktop_file = apps_dir.join("url-dispatcher.desktop");
    if desktop_file.exists() {
        std::fs::remove_file(&desktop_file)?;
    }

    let _ = std::process::Command::new("update-desktop-database")
        .arg(&apps_dir)
        .status();

    Ok(())
}

// ─── Windows：通过 HKCU 注册表注册 | Windows: register via HKCU registry ────

#[cfg(windows)]
pub fn register_as_default_browser(exe_path: &Path) -> Result<()> {
    use winreg::enums::*;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let exe_str = exe_path.display().to_string();

    // 创建 URL 协议处理类 | Create URL protocol handler class
    let (class_key, _) = hkcu.create_subkey("Software\\Classes\\URLDispatcherURL")?;
    class_key.set_value("", &"URL Dispatcher")?;
    class_key.set_value("URL Protocol", &"")?;

    let (cmd_key, _) =
        hkcu.create_subkey("Software\\Classes\\URLDispatcherURL\\shell\\open\\command")?;
    cmd_key.set_value("", &format!("\"{}\" \"%1\"", exe_str))?;

    // 注册为开始菜单浏览器客户端 | Register as a StartMenuInternet client
    let (client_key, _) =
        hkcu.create_subkey("Software\\Clients\\StartMenuInternet\\URLDispatcher")?;
    client_key.set_value("", &"URL Dispatcher")?;

    let (client_cmd, _) =
        client_key.create_subkey("shell\\open\\command")?;
    client_cmd.set_value("", &format!("\"{}\"", exe_str))?;

    // 应用能力声明 | Application capabilities
    let (cap_key, _) = client_key.create_subkey("Capabilities")?;
    cap_key.set_value("ApplicationName", &"URL Dispatcher")?;
    cap_key.set_value(
        "ApplicationDescription",
        &"Configurable URL dispatcher and browser selector",
    )?;

    let (urlassoc, _) = cap_key.create_subkey("URLAssociations")?;
    urlassoc.set_value("http", &"URLDispatcherURL")?;
    urlassoc.set_value("https", &"URLDispatcherURL")?;

    // 注册到已注册应用列表 | Register in RegisteredApplications
    let (regapps, _) =
        hkcu.create_subkey("Software\\RegisteredApplications")?;
    regapps.set_value(
        "URLDispatcher",
        &"Software\\Clients\\StartMenuInternet\\URLDispatcher\\Capabilities",
    )?;

    Ok(())
}

#[cfg(windows)]
pub fn unregister_as_default_browser() -> Result<()> {
    use winreg::enums::*;
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    let _ = hkcu.delete_subkey_all("Software\\Classes\\URLDispatcherURL");
    let _ = hkcu.delete_subkey_all("Software\\Clients\\StartMenuInternet\\URLDispatcher");

    if let Ok(regapps) =
        hkcu.open_subkey_with_flags("Software\\RegisteredApplications", KEY_WRITE)
    {
        let _ = regapps.delete_value("URLDispatcher");
    }

    Ok(())
}

// ─── 其他平台回退 | Fallback for other platforms ────────────────────────────

#[cfg(not(any(windows, target_os = "linux")))]
pub fn register_as_default_browser(_exe_path: &Path) -> Result<()> {
    Err(anyhow!("Default browser registration is not supported on this platform"))
}

#[cfg(not(any(windows, target_os = "linux")))]
pub fn unregister_as_default_browser() -> Result<()> {
    Err(anyhow!("Default browser unregistration is not supported on this platform"))
}
