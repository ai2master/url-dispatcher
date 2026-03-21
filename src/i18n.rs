/*
 * 国际化模块 | Internationalization Module
 *
 * 功能概述 | Overview:
 * 本模块负责 URL Dispatcher 的多语言支持，包括语言检测和 UI 字符串翻译。
 * 目前支持中文和英文两种语言。
 *
 * This module handles multilingual support for URL Dispatcher, including
 * language detection and UI string translation. Currently supports Chinese
 * and English.
 *
 * 设计说明 | Design Notes:
 * - 使用静态方法而非外部 i18n crate，简化依赖，减小二进制体积
 * - 所有翻译字符串集中在 Tr 结构体的静态方法中
 * - 使用 Unicode 转义序列存储中文字符，避免源文件编码问题
 * - 语言检测基于环境变量（LANG、LC_ALL 等）
 *
 * - Uses static methods instead of external i18n crate to simplify dependencies
 *   and reduce binary size
 * - All translation strings are centralized in static methods of Tr struct
 * - Uses Unicode escape sequences for Chinese characters to avoid source file
 *   encoding issues
 * - Language detection based on environment variables (LANG, LC_ALL, etc.)
 *
 * 关键概念 | Key Concepts:
 * - Language enum: 表示支持的语言类型
 *                 Represents supported language types
 * - detect_system_language: 根据系统环境自动检测语言
 *                          Auto-detects language based on system environment
 * - Tr struct: 提供所有 UI 字符串的翻译方法
 *             Provides translation methods for all UI strings
 */

use serde::{Deserialize, Serialize};

/// 语言枚举 | Language Enum
///
/// 定义 URL Dispatcher 支持的所有语言。
/// Defines all languages supported by URL Dispatcher.
///
/// 变体说明 | Variants:
/// - `English`: 英语
///             English
/// - `Chinese`: 简体中文
///             Simplified Chinese
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    English,
    Chinese,
}

impl Language {
    /// 获取语言的显示标签 | Get display label for the language
    ///
    /// 返回值 | Return:
    /// 语言的本地化名称，如"English"或"中文"。
    /// Localized name of the language, such as "English" or "中文".
    pub fn label(self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Chinese => "\u{4e2d}\u{6587}", // "中文"
        }
    }
}

/// 检测系统语言 | Detect system language
///
/// 通过检查常用的语言环境变量来自动检测系统语言。
/// 如果检测到中文环境，返回 Chinese；否则默认返回 English。
///
/// Automatically detects system language by checking common locale environment
/// variables. Returns Chinese if Chinese environment detected; otherwise defaults
/// to English.
///
/// 检测逻辑 | Detection Logic:
/// 1. 按优先级检查环境变量：LANG、LC_ALL、LC_MESSAGES、LANGUAGE
///    Check environment variables in priority order: LANG, LC_ALL, LC_MESSAGES, LANGUAGE
/// 2. 如果变量值以 "zh" 开头或包含 "chinese"，判定为中文环境
///    If variable value starts with "zh" or contains "chinese", determine as Chinese environment
/// 3. 否则默认使用英文
///    Otherwise default to English
///
/// 返回值 | Return:
/// 检测到的语言类型。
/// Detected language type.
///
/// 示例 | Examples:
/// - LANG=zh_CN.UTF-8 -> Language::Chinese
/// - LANG=en_US.UTF-8 -> Language::English
/// - LANG=ja_JP.UTF-8 -> Language::English (默认 | default)
pub fn detect_system_language() -> Language {
    // 按优先级检查常用的 locale 环境变量
    // Check common locale environment variables in priority order
    for var in &["LANG", "LC_ALL", "LC_MESSAGES", "LANGUAGE"] {
        if let Ok(val) = std::env::var(var) {
            let val_lower = val.to_lowercase();
            // 检测中文 locale：zh_CN、zh_TW、zh_HK 等
            // Detect Chinese locale: zh_CN, zh_TW, zh_HK, etc.
            if val_lower.starts_with("zh") || val_lower.contains("chinese") {
                return Language::Chinese;
            }
        }
    }
    // 默认使用英文 | Default to English
    Language::English
}

/// 翻译字符串结构体 | Translation Strings Structure
///
/// 集中管理所有 UI 字符串的翻译。每个方法返回指定语言的翻译字符串。
/// 使用静态方法而非实例方法，无需实例化即可使用。
///
/// Centrally manages translations for all UI strings. Each method returns
/// the translated string for the specified language. Uses static methods
/// instead of instance methods, can be used without instantiation.
///
/// 用途 | Usage:
/// ```rust
/// use url_dispatcher::i18n::{Language, Tr};
/// let lang = Language::Chinese;
/// let label = Tr::settings(lang);  // "设置"
/// ```
pub struct Tr;

impl Tr {
    // ═══════════════════════════════════════════════════════════════════════════
    // 分发界面字符串 | Dispatcher UI Strings
    // ═══════════════════════════════════════════════════════════════════════════

    /// URL 标签 | URL Label
    /// 用途：在分发界面顶部显示 URL 提示
    /// Usage: Display URL prompt at the top of dispatcher UI
    pub fn url_label(lang: Language) -> &'static str {
        match lang {
            Language::English => "URL:",
            Language::Chinese => "URL\u{ff1a}", // "URL："
        }
    }

    /// 设置按钮 | Settings Button
    /// 用途：打开设置界面的按钮文本
    /// Usage: Button text to open settings UI
    pub fn settings(lang: Language) -> &'static str {
        match lang {
            Language::English => "Settings",
            Language::Chinese => "\u{8bbe}\u{7f6e}", // "设置"
        }
    }

    /// 取消按钮 | Cancel Button
    /// 用途：关闭分发界面的按钮文本
    /// Usage: Button text to close dispatcher UI
    pub fn cancel(lang: Language) -> &'static str {
        match lang {
            Language::English => "Cancel",
            Language::Chinese => "\u{53d6}\u{6d88}", // "取消"
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // 设置界面字符串 | Settings UI Strings
    // ═══════════════════════════════════════════════════════════════════════════

    /// 设置界面标题 | Settings UI Title
    /// 用途：设置窗口的顶部标题
    /// Usage: Top title of settings window
    pub fn settings_title(lang: Language) -> &'static str {
        match lang {
            Language::English => "URL Dispatcher Settings",
            Language::Chinese => "URL Dispatcher \u{8bbe}\u{7f6e}",
        }
    }

    /// 动作列表标题 | Actions List Title
    /// 用途：动作列表区域的标题
    /// Usage: Title of actions list section
    pub fn actions(lang: Language) -> &'static str {
        match lang {
            Language::English => "Actions",
            Language::Chinese => "\u{52a8}\u{4f5c}\u{5217}\u{8868}", // "动作列表"
        }
    }

    /// 删除按钮 | Delete Button
    /// 用途：删除动作的按钮文本
    /// Usage: Button text to delete an action
    pub fn delete(lang: Language) -> &'static str {
        match lang {
            Language::English => "Delete",
            Language::Chinese => "\u{5220}\u{9664}", // "删除"
        }
    }

    /// 编辑按钮 | Edit Button
    /// 用途：编辑动作的按钮文本
    /// Usage: Button text to edit an action
    pub fn edit(lang: Language) -> &'static str {
        match lang {
            Language::English => "Edit",
            Language::Chinese => "\u{7f16}\u{8f91}", // "编辑"
        }
    }

    /// 上移按钮 | Move Up Button
    /// 用途：将动作在列表中上移的按钮文本
    /// Usage: Button text to move action up in list
    pub fn up(lang: Language) -> &'static str {
        match lang {
            Language::English => "Up",
            Language::Chinese => "\u{4e0a}\u{79fb}", // "上移"
        }
    }

    /// 下移按钮 | Move Down Button
    /// 用途：将动作在列表中下移的按钮文本
    /// Usage: Button text to move action down in list
    pub fn down(lang: Language) -> &'static str {
        match lang {
            Language::English => "Down",
            Language::Chinese => "\u{4e0b}\u{79fb}", // "下移"
        }
    }

    /// 添加动作按钮 | Add Action Button
    /// 用途：打开动作编辑器添加新动作的按钮文本
    /// Usage: Button text to open action editor to add new action
    pub fn add_action(lang: Language) -> &'static str {
        match lang {
            Language::English => "+ Add Action",
            Language::Chinese => "+ \u{6dfb}\u{52a0}\u{52a8}\u{4f5c}", // "+ 添加动作"
        }
    }

    pub fn append_file_path(lang: Language) -> &'static str {
        match lang {
            Language::English => "Append File Path",
            Language::Chinese => "\u{8ffd}\u{52a0}\u{6587}\u{4ef6}\u{8def}\u{5f84}",
        }
    }

    pub fn append_file_description(lang: Language) -> &'static str {
        match lang {
            Language::English => "URLs will be appended to this file when using 'Append to File' action:",
            Language::Chinese => "\u{4f7f}\u{7528}\u{201c}\u{8ffd}\u{52a0}\u{5230}\u{6587}\u{4ef6}\u{201d}\u{52a8}\u{4f5c}\u{65f6}\u{ff0c}URL \u{5c06}\u{88ab}\u{8ffd}\u{52a0}\u{5230}\u{6b64}\u{6587}\u{4ef6}\u{ff1a}",
        }
    }

    pub fn system_integration(lang: Language) -> &'static str {
        match lang {
            Language::English => "System Integration",
            Language::Chinese => "\u{7cfb}\u{7edf}\u{96c6}\u{6210}",
        }
    }

    pub fn register_default_browser(lang: Language) -> &'static str {
        match lang {
            Language::English => "Register as Default Browser",
            Language::Chinese => "\u{6ce8}\u{518c}\u{4e3a}\u{9ed8}\u{8ba4}\u{6d4f}\u{89c8}\u{5668}",
        }
    }

    pub fn unregister(lang: Language) -> &'static str {
        match lang {
            Language::English => "Unregister",
            Language::Chinese => "\u{53d6}\u{6d88}\u{6ce8}\u{518c}",
        }
    }

    #[cfg(windows)]
    pub fn windows_hint(lang: Language) -> &'static str {
        match lang {
            Language::English => "After registering, go to Windows Settings > Apps > Default apps > Web browser and select URL Dispatcher.",
            Language::Chinese => "\u{6ce8}\u{518c}\u{540e}\u{ff0c}\u{8bf7}\u{524d}\u{5f80} Windows \u{8bbe}\u{7f6e} > \u{5e94}\u{7528} > \u{9ed8}\u{8ba4}\u{5e94}\u{7528} > Web \u{6d4f}\u{89c8}\u{5668}\u{ff0c}\u{9009}\u{62e9} URL Dispatcher\u{3002}",
        }
    }

    pub fn save_configuration(lang: Language) -> &'static str {
        match lang {
            Language::English => "Save Configuration",
            Language::Chinese => "\u{4fdd}\u{5b58}\u{914d}\u{7f6e}",
        }
    }

    pub fn config_saved(lang: Language) -> &'static str {
        match lang {
            Language::English => "Configuration saved!",
            Language::Chinese => "\u{914d}\u{7f6e}\u{5df2}\u{4fdd}\u{5b58}\u{ff01}",
        }
    }

    pub fn save_failed(lang: Language, err: &str) -> String {
        match lang {
            Language::English => format!("Failed to save: {}", err),
            Language::Chinese => format!("\u{4fdd}\u{5b58}\u{5931}\u{8d25}\u{ff1a}{}", err),
        }
    }

    pub fn registered_ok(lang: Language) -> &'static str {
        match lang {
            Language::English => "Registered successfully!",
            Language::Chinese => "\u{6ce8}\u{518c}\u{6210}\u{529f}\u{ff01}",
        }
    }

    pub fn register_failed(lang: Language, err: &str) -> String {
        match lang {
            Language::English => format!("Registration failed: {}", err),
            Language::Chinese => format!("\u{6ce8}\u{518c}\u{5931}\u{8d25}\u{ff1a}{}", err),
        }
    }

    pub fn unregistered_ok(lang: Language) -> &'static str {
        match lang {
            Language::English => "Unregistered successfully!",
            Language::Chinese => "\u{53d6}\u{6d88}\u{6ce8}\u{518c}\u{6210}\u{529f}\u{ff01}",
        }
    }

    pub fn unregister_failed(lang: Language, err: &str) -> String {
        match lang {
            Language::English => format!("Unregistration failed: {}", err),
            Language::Chinese => format!(
                "\u{53d6}\u{6d88}\u{6ce8}\u{518c}\u{5931}\u{8d25}\u{ff1a}{}",
                err
            ),
        }
    }

    pub fn exe_path_error(lang: Language, err: &str) -> String {
        match lang {
            Language::English => format!("Cannot determine exe path: {}", err),
            Language::Chinese => format!(
                "\u{65e0}\u{6cd5}\u{786e}\u{5b9a}\u{7a0b}\u{5e8f}\u{8def}\u{5f84}\u{ff1a}{}",
                err
            ),
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // 动作编辑器字符串 | Action Editor Strings
    // ═══════════════════════════════════════════════════════════════════════════

    /// 编辑动作窗口标题 | Edit Action Window Title
    /// 用途：编辑已存在动作时的窗口标题
    /// Usage: Window title when editing an existing action
    pub fn edit_action(lang: Language) -> &'static str {
        match lang {
            Language::English => "Edit Action",
            Language::Chinese => "\u{7f16}\u{8f91}\u{52a8}\u{4f5c}",
        }
    }

    pub fn add_action_title(lang: Language) -> &'static str {
        match lang {
            Language::English => "Add Action",
            Language::Chinese => "\u{6dfb}\u{52a0}\u{52a8}\u{4f5c}",
        }
    }

    pub fn type_label(lang: Language) -> &'static str {
        match lang {
            Language::English => "Type:",
            Language::Chinese => "\u{7c7b}\u{578b}\u{ff1a}",
        }
    }

    pub fn name_label(lang: Language) -> &'static str {
        match lang {
            Language::English => "Name:",
            Language::Chinese => "\u{540d}\u{79f0}\u{ff1a}",
        }
    }

    pub fn executable_label(lang: Language) -> &'static str {
        match lang {
            Language::English => "Executable:",
            Language::Chinese => "\u{53ef}\u{6267}\u{884c}\u{6587}\u{4ef6}\u{ff1a}",
        }
    }

    pub fn arguments_label(lang: Language) -> &'static str {
        match lang {
            Language::English => "Arguments:",
            Language::Chinese => "\u{53c2}\u{6570}\u{ff1a}",
        }
    }

    pub fn args_hint(lang: Language) -> &'static str {
        match lang {
            Language::English => "Use {URL} as placeholder for the URL. Example: --incognito {URL}",
            Language::Chinese => "\u{7528} {URL} \u{4f5c}\u{4e3a} URL \u{5360}\u{4f4d}\u{7b26}\u{3002}\u{793a}\u{4f8b}\u{ff1a}--incognito {URL}",
        }
    }

    pub fn save(lang: Language) -> &'static str {
        match lang {
            Language::English => "Save",
            Language::Chinese => "\u{4fdd}\u{5b58}",
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // 动作类型标签 | Action Type Labels
    // ═══════════════════════════════════════════════════════════════════════════

    /// 复制到剪贴板动作标签 | Copy to Clipboard Action Label
    /// 用途：在动作列表和编辑器中显示的动作类型名称
    /// Usage: Action type name displayed in action list and editor
    pub fn copy_to_clipboard(lang: Language) -> &'static str {
        match lang {
            Language::English => "Copy to Clipboard",
            Language::Chinese => "\u{590d}\u{5236}\u{5230}\u{526a}\u{8d34}\u{677f}",
        }
    }

    pub fn append_to_file(lang: Language) -> &'static str {
        match lang {
            Language::English => "Append to File",
            Language::Chinese => "\u{8ffd}\u{52a0}\u{5230}\u{6587}\u{4ef6}",
        }
    }

    pub fn open_in_browser(lang: Language) -> &'static str {
        match lang {
            Language::English => "Open in Browser",
            Language::Chinese => "\u{5728}\u{6d4f}\u{89c8}\u{5668}\u{4e2d}\u{6253}\u{5f00}",
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // 错误消息 | Error Messages
    // ═══════════════════════════════════════════════════════════════════════════

    /// 追加文件路径未配置错误 | Append File Path Not Configured Error
    /// 用途：执行追加到文件动作时，如果路径未配置，显示此错误
    /// Usage: Display this error when executing append to file action if path not configured
    pub fn append_path_not_configured(lang: Language) -> &'static str {
        match lang {
            Language::English => "Append file path not configured. Please set it in Settings.",
            Language::Chinese => "\u{8ffd}\u{52a0}\u{6587}\u{4ef6}\u{8def}\u{5f84}\u{672a}\u{914d}\u{7f6e}\u{ff0c}\u{8bf7}\u{5728}\u{8bbe}\u{7f6e}\u{4e2d}\u{8bbe}\u{5b9a}\u{3002}",
        }
    }

    pub fn error_prefix(lang: Language, err: &str) -> String {
        match lang {
            Language::English => format!("Error: {}", err),
            Language::Chinese => format!("\u{9519}\u{8bef}\u{ff1a}{}", err),
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // 语言选择器 | Language Selector
    // ═══════════════════════════════════════════════════════════════════════════

    /// 语言标签 | Language Label
    /// 用途：语言下拉框的标签文本
    /// Usage: Label text for language dropdown
    pub fn language_label(lang: Language) -> &'static str {
        match lang {
            Language::English => "Language",
            Language::Chinese => "\u{8bed}\u{8a00}",
        }
    }
}
