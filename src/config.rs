// 配置管理：数据结构定义、加载和保存 | Configuration: data structures, loading and saving

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

use crate::i18n::{self, Language};

// 应用配置结构体 | Application configuration struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub version: u32,
    pub actions: Vec<Action>,
    pub append_file_path: Option<PathBuf>,
    #[serde(default = "default_language")]
    pub language: Language,
}

fn default_language() -> Language {
    i18n::detect_system_language()
}

// 动作枚举：复制、追加到文件、在浏览器中打开 | Action enum: copy, append to file, open in browser
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Action {
    CopyToClipboard {
        id: Uuid,
        name: String,
        enabled: bool,
    },
    AppendToFile {
        id: Uuid,
        name: String,
        enabled: bool,
    },
    OpenInBrowser {
        id: Uuid,
        name: String,
        enabled: bool,
        executable_path: String,
        args: Vec<String>,
    },
}

impl Action {
    pub fn id(&self) -> Uuid {
        match self {
            Action::CopyToClipboard { id, .. }
            | Action::AppendToFile { id, .. }
            | Action::OpenInBrowser { id, .. } => *id,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Action::CopyToClipboard { name, .. }
            | Action::AppendToFile { name, .. }
            | Action::OpenInBrowser { name, .. } => name,
        }
    }

    pub fn enabled(&self) -> bool {
        match self {
            Action::CopyToClipboard { enabled, .. }
            | Action::AppendToFile { enabled, .. }
            | Action::OpenInBrowser { enabled, .. } => *enabled,
        }
    }

    pub fn set_enabled(&mut self, value: bool) {
        match self {
            Action::CopyToClipboard { enabled, .. }
            | Action::AppendToFile { enabled, .. }
            | Action::OpenInBrowser { enabled, .. } => *enabled = value,
        }
    }

    pub fn type_label(&self, lang: Language) -> &str {
        match self {
            Action::CopyToClipboard { .. } => crate::i18n::Tr::copy_to_clipboard(lang),
            Action::AppendToFile { .. } => crate::i18n::Tr::append_to_file(lang),
            Action::OpenInBrowser { .. } => crate::i18n::Tr::open_in_browser(lang),
        }
    }
}

// 默认配置：包含复制和追加两个内置动作 | Default config with copy and append built-in actions
impl Default for Config {
    fn default() -> Self {
        let lang = i18n::detect_system_language();
        Self {
            version: 1,
            actions: vec![
                Action::CopyToClipboard {
                    id: Uuid::new_v4(),
                    name: i18n::Tr::copy_to_clipboard(lang).into(),
                    enabled: true,
                },
                Action::AppendToFile {
                    id: Uuid::new_v4(),
                    name: i18n::Tr::append_to_file(lang).into(),
                    enabled: true,
                },
            ],
            append_file_path: None,
            language: lang,
        }
    }
}

// 获取平台相关的配置目录 | Get platform-specific config directory
pub fn get_config_dir() -> Result<PathBuf> {
    let base = dirs::config_dir().context("Cannot determine config directory")?;
    let dir = if cfg!(windows) {
        base.join("URLDispatcher")
    } else {
        base.join("url-dispatcher")
    };
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub fn get_config_path() -> Result<PathBuf> {
    Ok(get_config_dir()?.join("config.json"))
}

// 加载配置，不存在则创建默认配置 | Load config, create default if not exists
pub fn load_config() -> Result<Config> {
    let path = get_config_path()?;
    if !path.exists() {
        let config = Config::default();
        save_config(&config)?;
        return Ok(config);
    }
    let contents = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read config from {}", path.display()))?;
    let config: Config = serde_json::from_str(&contents).unwrap_or_else(|_| Config::default());
    Ok(config)
}

// 保存配置到 JSON 文件 | Save config to JSON file
pub fn save_config(config: &Config) -> Result<()> {
    let path = get_config_path()?;
    let json = serde_json::to_string_pretty(config)?;
    std::fs::write(&path, json)?;
    Ok(())
}
