/*
 * 配置管理模块 | Configuration Management Module
 *
 * 功能概述 | Overview:
 * 本模块负责 URL Dispatcher 的配置管理，包括数据结构定义、序列化/反序列化、
 * 配置文件的加载和保存。配置采用 JSON 格式存储在用户配置目录中。
 *
 * This module handles configuration management for URL Dispatcher, including
 * data structure definitions, serialization/deserialization, and loading/saving
 * configuration files. Configurations are stored in JSON format in the user's
 * config directory.
 *
 * 配置文件位置 | Config File Location:
 * - Linux: ~/.config/url-dispatcher/config.json
 * - Windows: %APPDATA%\URLDispatcher\config.json
 * - macOS: ~/Library/Application Support/url-dispatcher/config.json
 *
 * 设计说明 | Design Notes:
 * - 使用 serde 进行 JSON 序列化，便于手动编辑和版本控制
 * - Action 使用标签式枚举（tagged enum）存储，type 字段用于区分类型
 * - 支持配置版本控制，便于未来升级和迁移
 * - 自动检测系统语言作为默认配置
 *
 * - Uses serde for JSON serialization, facilitating manual editing and version control
 * - Action uses tagged enum storage, with type field for type discrimination
 * - Supports config versioning for future upgrades and migrations
 * - Auto-detects system language as default configuration
 */

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

use crate::i18n::{self, Language};

/// 应用配置结构体 | Application Configuration Structure
///
/// 存储所有应用设置，包括动作列表、文件路径和用户界面语言。
/// 配置通过 serde 序列化为 JSON 格式保存到磁盘。
///
/// Stores all application settings including action list, file paths, and UI language.
/// Configuration is serialized to JSON format via serde and saved to disk.
///
/// 字段说明 | Fields:
/// - `version`: 配置格式版本号，用于未来的迁移和兼容性检查
///            Config format version number for future migration and compatibility checks
/// - `actions`: 用户定义的动作列表，按顺序显示在分发界面中
///            User-defined action list, displayed in order in the dispatcher UI
/// - `append_file_path`: "追加到文件"动作的目标文件路径（可选）
///                      Target file path for "Append to File" action (optional)
/// - `language`: 用户界面语言（中文/英文）
///              User interface language (Chinese/English)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub version: u32,
    pub actions: Vec<Action>,
    pub append_file_path: Option<PathBuf>,
    #[serde(default = "default_language")]
    pub language: Language,
}

/// 默认语言获取函数 | Default Language Getter
///
/// 用于 serde 反序列化时的默认值，自动检测系统语言。
/// Used as default value during serde deserialization, auto-detects system language.
fn default_language() -> Language {
    i18n::detect_system_language()
}

/// 动作枚举 | Action Enum
///
/// 定义 URL Dispatcher 支持的所有动作类型。每个动作都包含唯一 ID、
/// 显示名称和启用状态。使用 serde 的 tag 属性实现标签式枚举序列化。
///
/// Defines all action types supported by URL Dispatcher. Each action contains
/// a unique ID, display name, and enabled status. Uses serde's tag attribute
/// for tagged enum serialization.
///
/// 变体说明 | Variants:
///
/// - `CopyToClipboard`: 将 URL 复制到系统剪贴板
///                     Copy URL to system clipboard
///   - `id`: 唯一标识符，用于编辑和删除操作
///          Unique identifier for edit and delete operations
///   - `name`: 在 UI 中显示的动作名称
///            Action name displayed in UI
///   - `enabled`: 是否在分发界面中显示此动作
///               Whether to display this action in dispatcher UI
///
/// - `AppendToFile`: 将 URL（带时间戳）追加到指定文件
///                  Append URL (with timestamp) to specified file
///   - `id`: 唯一标识符
///          Unique identifier
///   - `name`: 动作名称
///            Action name
///   - `enabled`: 启用状态
///               Enabled status
///
/// - `OpenInBrowser`: 使用指定的浏览器和参数打开 URL
///                   Open URL with specified browser and arguments
///   - `id`: 唯一标识符
///          Unique identifier
///   - `name`: 动作名称
///            Action name
///   - `enabled`: 启用状态
///               Enabled status
///   - `executable_path`: 浏览器可执行文件的完整路径
///                       Full path to browser executable
///   - `args`: 命令行参数列表，支持 {URL} 占位符
///            Command line arguments list, supports {URL} placeholder
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
    /// 获取动作的唯一 ID | Get the unique ID of the action
    ///
    /// 返回值 | Return:
    /// 动作的 UUID，用于在配置中唯一标识此动作。
    /// The action's UUID, used to uniquely identify this action in the configuration.
    pub fn id(&self) -> Uuid {
        match self {
            Action::CopyToClipboard { id, .. }
            | Action::AppendToFile { id, .. }
            | Action::OpenInBrowser { id, .. } => *id,
        }
    }

    /// 获取动作的显示名称 | Get the display name of the action
    ///
    /// 返回值 | Return:
    /// 在 UI 中显示的动作名称字符串引用。
    /// String reference to the action name displayed in the UI.
    pub fn name(&self) -> &str {
        match self {
            Action::CopyToClipboard { name, .. }
            | Action::AppendToFile { name, .. }
            | Action::OpenInBrowser { name, .. } => name,
        }
    }

    /// 获取动作的启用状态 | Get the enabled status of the action
    ///
    /// 返回值 | Return:
    /// true 表示动作已启用，会在分发界面显示；false 表示已禁用，不会显示。
    /// true means action is enabled and will be displayed in dispatcher UI;
    /// false means disabled and will not be displayed.
    pub fn enabled(&self) -> bool {
        match self {
            Action::CopyToClipboard { enabled, .. }
            | Action::AppendToFile { enabled, .. }
            | Action::OpenInBrowser { enabled, .. } => *enabled,
        }
    }

    /// 设置动作的启用状态 | Set the enabled status of the action
    ///
    /// 参数 | Parameters:
    /// - `value`: 新的启用状态值
    ///           New enabled status value
    pub fn set_enabled(&mut self, value: bool) {
        match self {
            Action::CopyToClipboard { enabled, .. }
            | Action::AppendToFile { enabled, .. }
            | Action::OpenInBrowser { enabled, .. } => *enabled = value,
        }
    }

    /// 获取动作类型的本地化标签 | Get the localized label for the action type
    ///
    /// 参数 | Parameters:
    /// - `lang`: 目标语言（中文或英文）
    ///          Target language (Chinese or English)
    ///
    /// 返回值 | Return:
    /// 动作类型的本地化字符串，如"复制到剪贴板"或"Copy to Clipboard"。
    /// Localized string for the action type, such as "复制到剪贴板" or "Copy to Clipboard".
    pub fn type_label(&self, lang: Language) -> &str {
        match self {
            Action::CopyToClipboard { .. } => crate::i18n::Tr::copy_to_clipboard(lang),
            Action::AppendToFile { .. } => crate::i18n::Tr::append_to_file(lang),
            Action::OpenInBrowser { .. } => crate::i18n::Tr::open_in_browser(lang),
        }
    }
}

/// 默认配置实现 | Default Configuration Implementation
///
/// 当配置文件不存在时，自动创建包含两个基础动作的默认配置。
/// 动作名称根据检测到的系统语言自动本地化。
///
/// Automatically creates a default configuration with two basic actions
/// when the config file doesn't exist. Action names are automatically
/// localized based on detected system language.
///
/// 默认配置包含 | Default configuration includes:
/// 1. "复制到剪贴板"动作，默认启用
///    "Copy to Clipboard" action, enabled by default
/// 2. "追加到文件"动作，默认启用
///    "Append to File" action, enabled by default
/// 3. 空的追加文件路径（需要用户在设置中配置）
///    Empty append file path (requires user configuration in settings)
/// 4. 自动检测的系统语言
///    Auto-detected system language
impl Default for Config {
    fn default() -> Self {
        // 检测系统语言，用于生成本地化的默认动作名称
        // Detect system language for generating localized default action names
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

/// 获取平台相关的配置目录 | Get platform-specific config directory
///
/// 根据操作系统返回标准配置目录路径，并自动创建目录（如果不存在）。
/// Returns standard config directory path based on operating system,
/// and automatically creates the directory if it doesn't exist.
///
/// 路径规则 | Path Rules:
/// - Linux: ~/.config/url-dispatcher/
/// - Windows: %APPDATA%\URLDispatcher\
/// - macOS: ~/Library/Application Support/url-dispatcher/
///
/// 注意 Windows 使用大写混合命名，其他平台使用小写连字符命名。
/// Note that Windows uses mixed-case naming, while other platforms use lowercase hyphenated naming.
///
/// 返回值 | Return:
/// 配置目录的完整路径。
/// Full path to the configuration directory.
///
/// 错误 | Errors:
/// 如果无法确定基础配置目录或创建目录失败，返回错误。
/// Returns error if base config directory cannot be determined or directory creation fails.
pub fn get_config_dir() -> Result<PathBuf> {
    // 获取系统标准配置目录 | Get system standard config directory
    let base = dirs::config_dir().context("Cannot determine config directory")?;

    // Windows 使用大写混合命名，其他平台使用小写连字符
    // Windows uses mixed-case naming, other platforms use lowercase hyphenated
    let dir = if cfg!(windows) {
        base.join("URLDispatcher")
    } else {
        base.join("url-dispatcher")
    };

    // 确保目录存在 | Ensure directory exists
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

/// 获取配置文件的完整路径 | Get the full path to the config file
///
/// 返回值 | Return:
/// config.json 文件的完整路径。
/// Full path to the config.json file.
pub fn get_config_path() -> Result<PathBuf> {
    Ok(get_config_dir()?.join("config.json"))
}

/// 加载配置文件 | Load configuration file
///
/// 从磁盘加载配置文件。如果文件不存在，创建并保存默认配置。
/// 如果文件存在但解析失败，自动回退到默认配置。
///
/// Loads configuration file from disk. If file doesn't exist, creates and saves
/// default configuration. If file exists but parsing fails, automatically falls
/// back to default configuration.
///
/// 处理逻辑 | Processing Logic:
/// 1. 获取配置文件路径
///    Get config file path
/// 2. 如果文件不存在，创建默认配置并保存
///    If file doesn't exist, create and save default config
/// 3. 读取文件内容
///    Read file contents
/// 4. 尝试解析 JSON，失败则使用默认配置
///    Try to parse JSON, use default config if parsing fails
///
/// 返回值 | Return:
/// 加载或创建的配置对象。
/// The loaded or created configuration object.
pub fn load_config() -> Result<Config> {
    let path = get_config_path()?;

    // 配置文件不存在，创建默认配置
    // Config file doesn't exist, create default config
    if !path.exists() {
        let config = Config::default();
        save_config(&config)?;
        return Ok(config);
    }

    // 读取并解析配置文件
    // Read and parse config file
    let contents = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read config from {}", path.display()))?;

    // 解析失败时回退到默认配置（保护性措施）
    // Fall back to default config on parsing failure (defensive measure)
    let config: Config = serde_json::from_str(&contents).unwrap_or_else(|_| Config::default());
    Ok(config)
}

/// 保存配置到文件 | Save configuration to file
///
/// 将配置对象序列化为格式化的 JSON 并写入磁盘。
/// Serializes configuration object to formatted JSON and writes to disk.
///
/// 参数 | Parameters:
/// - `config`: 要保存的配置对象引用
///            Reference to the configuration object to save
///
/// 返回值 | Return:
/// 成功返回 Ok(())，失败返回错误。
/// Returns Ok(()) on success, error on failure.
pub fn save_config(config: &Config) -> Result<()> {
    let path = get_config_path()?;
    // 使用 pretty 格式化，便于人工编辑
    // Use pretty formatting for human readability
    let json = serde_json::to_string_pretty(config)?;
    std::fs::write(&path, json)?;
    Ok(())
}
