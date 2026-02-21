use anyhow::{anyhow, Result};
use std::path::Path;

pub fn copy_to_clipboard(url: &str) -> Result<()> {
    let mut clipboard = arboard::Clipboard::new()?;
    clipboard.set_text(url)?;
    Ok(())
}

pub fn append_to_file(url: &str, file_path: &Path) -> Result<()> {
    use std::fs::OpenOptions;
    use std::io::Write;

    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    writeln!(file, "[{}] {}", timestamp, url)?;
    Ok(())
}

pub fn open_in_browser(url: &str, executable: &str, args_template: &[String]) -> Result<()> {
    let args: Vec<String> = if args_template.is_empty() {
        vec![url.to_string()]
    } else {
        args_template
            .iter()
            .map(|arg| arg.replace("{URL}", url))
            .collect()
    };

    let child = std::process::Command::new(executable).args(&args).spawn();

    match child {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow!("Failed to launch '{}': {}", executable, e)),
    }
}
