use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    English,
    Chinese,
}

impl Language {
    pub fn label(self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Chinese => "\u{4e2d}\u{6587}",
        }
    }
}

/// Detect system language. Returns Chinese if locale contains "zh", otherwise English.
pub fn detect_system_language() -> Language {
    // Check common environment variables for locale
    for var in &["LANG", "LC_ALL", "LC_MESSAGES", "LANGUAGE"] {
        if let Ok(val) = std::env::var(var) {
            let val_lower = val.to_lowercase();
            if val_lower.starts_with("zh") || val_lower.contains("chinese") {
                return Language::Chinese;
            }
        }
    }
    Language::English
}

/// All translatable UI strings
pub struct Tr;

impl Tr {
    // ── Dispatcher UI ────────────────────────────────────────
    pub fn url_label(lang: Language) -> &'static str {
        match lang {
            Language::English => "URL:",
            Language::Chinese => "URL\u{ff1a}",
        }
    }

    pub fn settings(lang: Language) -> &'static str {
        match lang {
            Language::English => "Settings",
            Language::Chinese => "\u{8bbe}\u{7f6e}",
        }
    }

    pub fn cancel(lang: Language) -> &'static str {
        match lang {
            Language::English => "Cancel",
            Language::Chinese => "\u{53d6}\u{6d88}",
        }
    }

    // ── Settings UI ──────────────────────────────────────────
    pub fn settings_title(lang: Language) -> &'static str {
        match lang {
            Language::English => "URL Dispatcher Settings",
            Language::Chinese => "URL Dispatcher \u{8bbe}\u{7f6e}",
        }
    }

    pub fn actions(lang: Language) -> &'static str {
        match lang {
            Language::English => "Actions",
            Language::Chinese => "\u{52a8}\u{4f5c}\u{5217}\u{8868}",
        }
    }

    pub fn delete(lang: Language) -> &'static str {
        match lang {
            Language::English => "Delete",
            Language::Chinese => "\u{5220}\u{9664}",
        }
    }

    pub fn edit(lang: Language) -> &'static str {
        match lang {
            Language::English => "Edit",
            Language::Chinese => "\u{7f16}\u{8f91}",
        }
    }

    pub fn up(lang: Language) -> &'static str {
        match lang {
            Language::English => "Up",
            Language::Chinese => "\u{4e0a}\u{79fb}",
        }
    }

    pub fn down(lang: Language) -> &'static str {
        match lang {
            Language::English => "Down",
            Language::Chinese => "\u{4e0b}\u{79fb}",
        }
    }

    pub fn add_action(lang: Language) -> &'static str {
        match lang {
            Language::English => "+ Add Action",
            Language::Chinese => "+ \u{6dfb}\u{52a0}\u{52a8}\u{4f5c}",
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
            Language::Chinese => format!("\u{53d6}\u{6d88}\u{6ce8}\u{518c}\u{5931}\u{8d25}\u{ff1a}{}", err),
        }
    }

    pub fn exe_path_error(lang: Language, err: &str) -> String {
        match lang {
            Language::English => format!("Cannot determine exe path: {}", err),
            Language::Chinese => format!("\u{65e0}\u{6cd5}\u{786e}\u{5b9a}\u{7a0b}\u{5e8f}\u{8def}\u{5f84}\u{ff1a}{}", err),
        }
    }

    // ── Action editor ────────────────────────────────────────
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

    // ── Action type labels ───────────────────────────────────
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

    // ── Error messages ───────────────────────────────────────
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

    // ── Language selector ────────────────────────────────────
    pub fn language_label(lang: Language) -> &'static str {
        match lang {
            Language::English => "Language",
            Language::Chinese => "\u{8bed}\u{8a00}",
        }
    }
}
