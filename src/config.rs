use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

use crate::i18n::{self, Language};

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

pub fn save_config(config: &Config) -> Result<()> {
    let path = get_config_path()?;
    let json = serde_json::to_string_pretty(config)?;
    std::fs::write(&path, json)?;
    Ok(())
}
