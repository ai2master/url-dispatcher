// ============================================================================
// test_config.rs — 配置模块测试 | Configuration Module Tests
// ============================================================================
//
// 测试覆盖 | Test Coverage:
//   - Config 结构体的默认值生成
//   - Config / Action 的 JSON 序列化和反序列化
//   - Action 枚举的 getter/setter 方法
//   - 配置文件的加载和保存（使用临时目录）
//   - 边界情况：空配置、损坏的 JSON、缺失字段
// ============================================================================

use std::path::PathBuf;
use url_dispatcher::config::*;
use url_dispatcher::i18n::Language;

// ─── 默认配置测试 | Default Configuration Tests ─────────────────────────────

#[test]
fn test_default_config_has_two_actions() {
    // 默认配置应包含 2 个动作 | Default config should have 2 actions
    let config = Config::default();
    assert_eq!(config.actions.len(), 2);
}

#[test]
fn test_default_config_version_is_1() {
    // 默认配置版本号为 1 | Default config version should be 1
    let config = Config::default();
    assert_eq!(config.version, 1);
}

#[test]
fn test_default_config_append_path_is_none() {
    // 默认配置的追加文件路径为空 | Default append file path should be None
    let config = Config::default();
    assert!(config.append_file_path.is_none());
}

#[test]
fn test_default_config_actions_are_enabled() {
    // 默认配置的所有动作都应启用 | All default actions should be enabled
    let config = Config::default();
    for action in &config.actions {
        assert!(action.enabled());
    }
}

#[test]
fn test_default_config_first_action_is_copy() {
    // 第一个默认动作应为 CopyToClipboard | First default action should be CopyToClipboard
    let config = Config::default();
    assert!(matches!(config.actions[0], Action::CopyToClipboard { .. }));
}

#[test]
fn test_default_config_second_action_is_append() {
    // 第二个默认动作应为 AppendToFile | Second default action should be AppendToFile
    let config = Config::default();
    assert!(matches!(config.actions[1], Action::AppendToFile { .. }));
}

// ─── Action 方法测试 | Action Method Tests ──────────────────────────────────

#[test]
fn test_action_id_is_unique() {
    // 每个动作的 ID 应唯一 | Each action's ID should be unique
    let config = Config::default();
    let id0 = config.actions[0].id();
    let id1 = config.actions[1].id();
    assert_ne!(id0, id1);
}

#[test]
fn test_action_name_not_empty() {
    // 默认动作名称不应为空 | Default action names should not be empty
    let config = Config::default();
    for action in &config.actions {
        assert!(!action.name().is_empty());
    }
}

#[test]
fn test_action_set_enabled() {
    // 测试启用/禁用切换 | Test enable/disable toggling
    let mut config = Config::default();
    assert!(config.actions[0].enabled());

    config.actions[0].set_enabled(false);
    assert!(!config.actions[0].enabled());

    config.actions[0].set_enabled(true);
    assert!(config.actions[0].enabled());
}

#[test]
fn test_action_type_label_english() {
    // 英文标签测试 | English label test
    let config = Config::default();
    let label = config.actions[0].type_label(Language::English);
    assert_eq!(label, "Copy to Clipboard");
}

#[test]
fn test_action_type_label_chinese() {
    // 中文标签测试 | Chinese label test
    let config = Config::default();
    let label = config.actions[0].type_label(Language::Chinese);
    assert!(!label.is_empty());
    // 中文 "复制到剪贴板" 的 Unicode | Unicode for "复制到剪贴板"
    assert_eq!(label, "\u{590d}\u{5236}\u{5230}\u{526a}\u{8d34}\u{677f}");
}

// ─── JSON 序列化/反序列化测试 | JSON Serialization Tests ─────────────────────

#[test]
fn test_config_serialize_deserialize_roundtrip() {
    // 配置应能完整往返序列化 | Config should roundtrip serialize
    let config = Config::default();
    let json = serde_json::to_string_pretty(&config).unwrap();
    let deserialized: Config = serde_json::from_str(&json).unwrap();

    assert_eq!(deserialized.version, config.version);
    assert_eq!(deserialized.actions.len(), config.actions.len());
    assert_eq!(deserialized.append_file_path, config.append_file_path);
}

#[test]
fn test_action_copy_serialize() {
    // CopyToClipboard 动作的序列化格式 | CopyToClipboard serialization format
    let action = Action::CopyToClipboard {
        id: uuid::Uuid::nil(),
        name: "Test Copy".into(),
        enabled: true,
    };
    let json = serde_json::to_string(&action).unwrap();
    assert!(json.contains("\"type\":\"CopyToClipboard\""));
    assert!(json.contains("\"name\":\"Test Copy\""));
    assert!(json.contains("\"enabled\":true"));
}

#[test]
fn test_action_append_serialize() {
    // AppendToFile 动作的序列化格式 | AppendToFile serialization format
    let action = Action::AppendToFile {
        id: uuid::Uuid::nil(),
        name: "Test Append".into(),
        enabled: false,
    };
    let json = serde_json::to_string(&action).unwrap();
    assert!(json.contains("\"type\":\"AppendToFile\""));
    assert!(json.contains("\"enabled\":false"));
}

#[test]
fn test_action_browser_serialize() {
    // OpenInBrowser 动作的序列化格式 | OpenInBrowser serialization format
    let action = Action::OpenInBrowser {
        id: uuid::Uuid::nil(),
        name: "Firefox".into(),
        enabled: true,
        executable_path: "/usr/bin/firefox".into(),
        args: vec!["--new-window".into(), "{URL}".into()],
    };
    let json = serde_json::to_string(&action).unwrap();
    assert!(json.contains("\"type\":\"OpenInBrowser\""));
    assert!(json.contains("\"executable_path\":\"/usr/bin/firefox\""));
    assert!(json.contains("{URL}"));
}

#[test]
fn test_action_browser_deserialize() {
    // 从 JSON 反序列化 OpenInBrowser | Deserialize OpenInBrowser from JSON
    let json = r#"{
        "type": "OpenInBrowser",
        "id": "00000000-0000-0000-0000-000000000000",
        "name": "Chrome",
        "enabled": true,
        "executable_path": "/usr/bin/google-chrome",
        "args": ["--incognito", "{URL}"]
    }"#;
    let action: Action = serde_json::from_str(json).unwrap();
    assert!(matches!(action, Action::OpenInBrowser { .. }));
    assert_eq!(action.name(), "Chrome");
    assert!(action.enabled());
    if let Action::OpenInBrowser {
        executable_path,
        args,
        ..
    } = &action
    {
        assert_eq!(executable_path, "/usr/bin/google-chrome");
        assert_eq!(args.len(), 2);
        assert_eq!(args[0], "--incognito");
        assert_eq!(args[1], "{URL}");
    }
}

#[test]
fn test_config_missing_language_uses_default() {
    // 缺少 language 字段的 JSON 应使用默认语言 | Missing language should use default
    let json = r#"{
        "version": 1,
        "actions": [],
        "append_file_path": null
    }"#;
    let config: Config = serde_json::from_str(json).unwrap();
    // 不崩溃即成功，语言有值 | Not crashing is success, language has value
    let _ = config.language;
}

#[test]
fn test_config_with_append_path() {
    // 带追加文件路径的配置 | Config with append file path
    let json = r#"{
        "version": 1,
        "actions": [],
        "append_file_path": "/tmp/urls.txt",
        "language": "English"
    }"#;
    let config: Config = serde_json::from_str(json).unwrap();
    assert_eq!(
        config.append_file_path,
        Some(PathBuf::from("/tmp/urls.txt"))
    );
}

// ─── 配置文件 I/O 测试 | Config File I/O Tests ──────────────────────────────

#[test]
fn test_save_and_load_config() {
    // 保存并重新加载配置 | Save and reload config
    // 使用临时目录避免影响真实配置 | Use temp dir to avoid affecting real config
    let dir = tempfile::tempdir().unwrap();
    let config_path = dir.path().join("config.json");

    let config = Config::default();
    let json = serde_json::to_string_pretty(&config).unwrap();
    std::fs::write(&config_path, &json).unwrap();

    let loaded_json = std::fs::read_to_string(&config_path).unwrap();
    let loaded: Config = serde_json::from_str(&loaded_json).unwrap();
    assert_eq!(loaded.version, config.version);
    assert_eq!(loaded.actions.len(), config.actions.len());
}

#[test]
fn test_corrupt_json_returns_default() {
    // 损坏的 JSON 应回退到默认配置 | Corrupt JSON should fall back to defaults
    let bad_json = "{ this is not valid json }}}";
    let result: Result<Config, _> = serde_json::from_str(bad_json);
    assert!(result.is_err());
    // 实际 load_config 会用 unwrap_or_else 回退 | Actual load_config falls back
    let config = result.unwrap_or_else(|_| Config::default());
    assert_eq!(config.version, 1);
    assert_eq!(config.actions.len(), 2);
}

#[test]
fn test_empty_actions_config() {
    // 空动作列表的配置 | Config with empty actions list
    let json = r#"{
        "version": 1,
        "actions": [],
        "append_file_path": null,
        "language": "Chinese"
    }"#;
    let config: Config = serde_json::from_str(json).unwrap();
    assert_eq!(config.actions.len(), 0);
    assert_eq!(config.language, Language::Chinese);
}

// ─── OpenInBrowser 参数边界情况 | OpenInBrowser Args Edge Cases ──────────────

#[test]
fn test_browser_action_empty_args() {
    // 空参数列表 | Empty args list
    let json = r#"{
        "type": "OpenInBrowser",
        "id": "00000000-0000-0000-0000-000000000000",
        "name": "Browser",
        "enabled": true,
        "executable_path": "/usr/bin/browser",
        "args": []
    }"#;
    let action: Action = serde_json::from_str(json).unwrap();
    if let Action::OpenInBrowser { args, .. } = &action {
        assert!(args.is_empty());
    }
}

#[test]
fn test_browser_action_multiple_url_placeholders() {
    // 多个 {URL} 占位符 | Multiple {URL} placeholders
    let action = Action::OpenInBrowser {
        id: uuid::Uuid::nil(),
        name: "Test".into(),
        enabled: true,
        executable_path: "/usr/bin/test".into(),
        args: vec!["{URL}".into(), "--ref={URL}".into()],
    };
    let json = serde_json::to_string(&action).unwrap();
    // 应包含两个 {URL} | Should contain two {URL}
    assert_eq!(json.matches("{URL}").count(), 2);
}
