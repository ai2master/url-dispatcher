// ============================================================================
// test_i18n.rs — 国际化模块测试 | Internationalization Module Tests
// ============================================================================

use url_dispatcher::i18n::{Language, Tr};

// ─── Language 枚举测试 | Language Enum Tests ─────────────────────────────────

#[test]
fn test_language_english_label() {
    assert_eq!(Language::English.label(), "English");
}

#[test]
fn test_language_chinese_label() {
    assert_eq!(Language::Chinese.label(), "\u{4e2d}\u{6587}"); // "中文"
}

#[test]
fn test_language_equality() {
    assert_eq!(Language::English, Language::English);
    assert_eq!(Language::Chinese, Language::Chinese);
    assert_ne!(Language::English, Language::Chinese);
}

#[test]
fn test_language_clone_copy() {
    let lang = Language::Chinese;
    let cloned = lang;
    assert_eq!(lang, cloned);
}

// ─── Language 序列化测试 | Language Serialization Tests ──────────────────────

#[test]
fn test_language_serialize_english() {
    let json = serde_json::to_string(&Language::English).unwrap();
    assert_eq!(json, "\"English\"");
}

#[test]
fn test_language_serialize_chinese() {
    let json = serde_json::to_string(&Language::Chinese).unwrap();
    assert_eq!(json, "\"Chinese\"");
}

#[test]
fn test_language_deserialize() {
    let en: Language = serde_json::from_str("\"English\"").unwrap();
    assert_eq!(en, Language::English);
    let zh: Language = serde_json::from_str("\"Chinese\"").unwrap();
    assert_eq!(zh, Language::Chinese);
}

// ─── Tr 翻译方法测试 | Tr Translation Method Tests ──────────────────────────

#[test]
fn test_tr_url_label() {
    assert_eq!(Tr::url_label(Language::English), "URL:");
    assert!(!Tr::url_label(Language::Chinese).is_empty());
}

#[test]
fn test_tr_settings() {
    assert_eq!(Tr::settings(Language::English), "Settings");
    assert!(!Tr::settings(Language::Chinese).is_empty());
}

#[test]
fn test_tr_cancel() {
    assert_eq!(Tr::cancel(Language::English), "Cancel");
    assert!(!Tr::cancel(Language::Chinese).is_empty());
}

#[test]
fn test_tr_settings_title() {
    let en = Tr::settings_title(Language::English);
    assert!(en.contains("URL Dispatcher"));
    let zh = Tr::settings_title(Language::Chinese);
    assert!(zh.contains("URL Dispatcher"));
}

#[test]
fn test_tr_actions() {
    assert_eq!(Tr::actions(Language::English), "Actions");
    assert!(!Tr::actions(Language::Chinese).is_empty());
}

#[test]
fn test_tr_action_buttons() {
    // 所有按钮标签不应为空 | All button labels should not be empty
    for lang in [Language::English, Language::Chinese] {
        assert!(!Tr::delete(lang).is_empty());
        assert!(!Tr::edit(lang).is_empty());
        assert!(!Tr::up(lang).is_empty());
        assert!(!Tr::down(lang).is_empty());
        assert!(!Tr::add_action(lang).is_empty());
        assert!(!Tr::save(lang).is_empty());
    }
}

#[test]
fn test_tr_action_type_labels() {
    // 动作类型标签 | Action type labels
    for lang in [Language::English, Language::Chinese] {
        assert!(!Tr::copy_to_clipboard(lang).is_empty());
        assert!(!Tr::append_to_file(lang).is_empty());
        assert!(!Tr::open_in_browser(lang).is_empty());
    }
}

#[test]
fn test_tr_action_type_labels_english_values() {
    assert_eq!(
        Tr::copy_to_clipboard(Language::English),
        "Copy to Clipboard"
    );
    assert_eq!(Tr::append_to_file(Language::English), "Append to File");
    assert_eq!(Tr::open_in_browser(Language::English), "Open in Browser");
}

#[test]
fn test_tr_error_messages() {
    // 错误消息格式 | Error message formats
    let en = Tr::error_prefix(Language::English, "test error");
    assert!(en.contains("test error"));
    assert!(en.starts_with("Error:"));

    let zh = Tr::error_prefix(Language::Chinese, "test error");
    assert!(zh.contains("test error"));
}

#[test]
fn test_tr_save_failed() {
    let en = Tr::save_failed(Language::English, "disk full");
    assert!(en.contains("disk full"));
    let zh = Tr::save_failed(Language::Chinese, "disk full");
    assert!(zh.contains("disk full"));
}

#[test]
fn test_tr_register_failed() {
    let en = Tr::register_failed(Language::English, "permission denied");
    assert!(en.contains("permission denied"));
}

#[test]
fn test_tr_config_related() {
    for lang in [Language::English, Language::Chinese] {
        assert!(!Tr::append_file_path(lang).is_empty());
        assert!(!Tr::append_file_description(lang).is_empty());
        assert!(!Tr::system_integration(lang).is_empty());
        assert!(!Tr::register_default_browser(lang).is_empty());
        assert!(!Tr::unregister(lang).is_empty());
        assert!(!Tr::save_configuration(lang).is_empty());
        assert!(!Tr::config_saved(lang).is_empty());
        assert!(!Tr::registered_ok(lang).is_empty());
        assert!(!Tr::unregistered_ok(lang).is_empty());
        assert!(!Tr::append_path_not_configured(lang).is_empty());
        assert!(!Tr::language_label(lang).is_empty());
    }
}

#[test]
fn test_tr_editor_labels() {
    for lang in [Language::English, Language::Chinese] {
        assert!(!Tr::edit_action(lang).is_empty());
        assert!(!Tr::add_action_title(lang).is_empty());
        assert!(!Tr::type_label(lang).is_empty());
        assert!(!Tr::name_label(lang).is_empty());
        assert!(!Tr::executable_label(lang).is_empty());
        assert!(!Tr::arguments_label(lang).is_empty());
        assert!(!Tr::args_hint(lang).is_empty());
    }
}

#[test]
fn test_tr_args_hint_contains_url_placeholder() {
    // 参数提示应包含 {URL} | Args hint should contain {URL}
    assert!(Tr::args_hint(Language::English).contains("{URL}"));
    assert!(Tr::args_hint(Language::Chinese).contains("{URL}"));
}

// ─── detect_system_language 测试 | detect_system_language Tests ──────────────

#[test]
fn test_detect_system_language_returns_valid() {
    // 应返回有效的语言枚举 | Should return valid language enum
    let lang = url_dispatcher::i18n::detect_system_language();
    // 不崩溃，且返回的语言有对应的标签 | Doesn't crash, and returned language has a label
    assert!(!lang.label().is_empty());
}
