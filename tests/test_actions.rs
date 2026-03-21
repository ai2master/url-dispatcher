// ============================================================================
// test_actions.rs — 动作执行模块测试 | Action Execution Module Tests
// ============================================================================
//
// 测试覆盖 | Test Coverage:
//   - append_to_file: 文件创建、追加、时间戳格式、目录自动创建
//   - open_in_browser: {URL} 占位符替换、空参数处理
//   - copy_to_clipboard: 在 CI 中需要 Xvfb（仅在有显示环境时测试）
// ============================================================================

use url_dispatcher::actions;

// ─── append_to_file 测试 | append_to_file Tests ─────────────────────────────

#[test]
fn test_append_to_file_creates_file() {
    // 追加到不存在的文件，应自动创建 | Append to nonexistent file, should auto-create
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("urls.txt");
    assert!(!file_path.exists());

    actions::append_to_file("https://example.com", &file_path).unwrap();
    assert!(file_path.exists());
}

#[test]
fn test_append_to_file_content_format() {
    // 验证写入格式：[时间戳] URL | Verify format: [timestamp] URL
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("urls.txt");

    actions::append_to_file("https://example.com", &file_path).unwrap();

    let content = std::fs::read_to_string(&file_path).unwrap();
    // 应以 [ 开头，包含 URL | Should start with [ and contain URL
    assert!(content.starts_with('['));
    assert!(content.contains("https://example.com"));
    // 应以换行结尾 | Should end with newline
    assert!(content.ends_with('\n'));
}

#[test]
fn test_append_to_file_timestamp_format() {
    // 验证时间戳格式 YYYY-MM-DD HH:MM:SS | Verify timestamp format
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("urls.txt");

    actions::append_to_file("https://test.org", &file_path).unwrap();

    let content = std::fs::read_to_string(&file_path).unwrap();
    // 提取方括号内的时间戳 | Extract timestamp inside brackets
    let bracket_end = content.find(']').unwrap();
    let timestamp = &content[1..bracket_end];
    // 时间戳应为 19 个字符: YYYY-MM-DD HH:MM:SS | Timestamp should be 19 chars
    assert_eq!(timestamp.len(), 19);
    // 第5和第8字符应为 '-' | 5th and 8th chars should be '-'
    assert_eq!(timestamp.as_bytes()[4], b'-');
    assert_eq!(timestamp.as_bytes()[7], b'-');
    // 第11字符应为空格 | 11th char should be space
    assert_eq!(timestamp.as_bytes()[10], b' ');
}

#[test]
fn test_append_to_file_multiple_appends() {
    // 多次追加不覆盖 | Multiple appends don't overwrite
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("urls.txt");

    actions::append_to_file("https://first.com", &file_path).unwrap();
    actions::append_to_file("https://second.com", &file_path).unwrap();
    actions::append_to_file("https://third.com", &file_path).unwrap();

    let content = std::fs::read_to_string(&file_path).unwrap();
    let lines: Vec<&str> = content.lines().collect();
    assert_eq!(lines.len(), 3);
    assert!(lines[0].contains("https://first.com"));
    assert!(lines[1].contains("https://second.com"));
    assert!(lines[2].contains("https://third.com"));
}

#[test]
fn test_append_to_file_creates_parent_dirs() {
    // 自动创建不存在的父目录 | Auto-creates nonexistent parent directories
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("nested").join("deep").join("urls.txt");
    assert!(!file_path.parent().unwrap().exists());

    actions::append_to_file("https://nested.com", &file_path).unwrap();
    assert!(file_path.exists());
}

#[test]
fn test_append_to_file_special_chars_in_url() {
    // URL 中的特殊字符应原样保留 | Special chars in URL should be preserved
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("urls.txt");

    let url = "https://example.com/search?q=hello+world&lang=zh_CN#section";
    actions::append_to_file(url, &file_path).unwrap();

    let content = std::fs::read_to_string(&file_path).unwrap();
    assert!(content.contains(url));
}

#[test]
fn test_append_to_file_empty_url() {
    // 空 URL 也应正常工作 | Empty URL should also work
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("urls.txt");

    actions::append_to_file("", &file_path).unwrap();

    let content = std::fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("] \n") || content.contains("] \r\n"));
}

#[test]
fn test_append_to_file_unicode_url() {
    // Unicode URL 应正常处理 | Unicode URL should be handled properly
    let dir = tempfile::tempdir().unwrap();
    let file_path = dir.path().join("urls.txt");

    let url = "https://example.com/路径/文件";
    actions::append_to_file(url, &file_path).unwrap();

    let content = std::fs::read_to_string(&file_path).unwrap();
    assert!(content.contains(url));
}

// ─── open_in_browser 参数处理测试 | open_in_browser Arg Processing Tests ────

#[test]
fn test_open_in_browser_nonexistent_binary() {
    // 不存在的可执行文件应返回错误 | Nonexistent executable should return error
    let result = actions::open_in_browser(
        "https://example.com",
        "/nonexistent/path/to/browser",
        &["{URL}".to_string()],
    );
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("/nonexistent/path/to/browser"));
}

#[test]
fn test_open_in_browser_empty_executable() {
    // 空可执行路径应返回错误 | Empty executable path should return error
    let result = actions::open_in_browser("https://example.com", "", &["{URL}".to_string()]);
    assert!(result.is_err());
}

// ─── copy_to_clipboard 测试（需要显示环境）| clipboard tests (needs display) ─

#[test]
fn test_copy_to_clipboard_no_crash() {
    // 在无显示环境中可能失败，但不应 panic
    // May fail without display, but should not panic
    let result = actions::copy_to_clipboard("https://example.com");
    // 不检查结果，只确保不 panic | Don't check result, just ensure no panic
    let _ = result;
}

// ─── App 和 ActionEditor 单元测试 | App and ActionEditor Unit Tests ─────────

#[test]
fn test_app_new_dispatch_mode() {
    use url_dispatcher::app::{App, AppMode};
    use url_dispatcher::config::Config;

    let config = Config::default();
    let app = App::new(AppMode::Dispatch("https://test.com".into()), config);
    assert!(!app.should_close);
    assert!(app.status_message.is_none());
    assert!(!app.status_is_error);
}

#[test]
fn test_app_new_settings_mode() {
    use url_dispatcher::app::{App, AppMode};
    use url_dispatcher::config::Config;

    let config = Config::default();
    let app = App::new(AppMode::Settings, config);
    assert!(!app.should_close);
}

#[test]
fn test_action_editor_default() {
    use url_dispatcher::ui_settings::ActionEditor;

    let editor = ActionEditor::default();
    assert!(!editor.active);
    assert!(editor.editing_id.is_none());
    assert!(editor.name.is_empty());
    assert_eq!(editor.args_str, "{URL}");
}

#[test]
fn test_action_editor_open_new() {
    use url_dispatcher::ui_settings::ActionEditor;

    let mut editor = ActionEditor::default();
    editor.open_new();
    assert!(editor.active);
    assert!(editor.editing_id.is_none());
}

#[test]
fn test_action_editor_open_edit() {
    use url_dispatcher::config::Action;
    use url_dispatcher::ui_settings::ActionEditor;

    let mut editor = ActionEditor::default();
    let action = Action::OpenInBrowser {
        id: uuid::Uuid::new_v4(),
        name: "Firefox".into(),
        enabled: true,
        executable_path: "/usr/bin/firefox".into(),
        args: vec!["--private".into(), "{URL}".into()],
    };

    editor.open_edit(&action);
    assert!(editor.active);
    assert!(editor.editing_id.is_some());
    assert_eq!(editor.name, "Firefox");
    assert_eq!(editor.executable_path, "/usr/bin/firefox");
    assert_eq!(editor.args_str, "--private {URL}");
}

#[test]
fn test_action_editor_build_copy_action() {
    use url_dispatcher::ui_settings::{ActionEditor, ActionTypeChoice};

    let editor = ActionEditor {
        action_type: ActionTypeChoice::CopyToClipboard,
        name: "Test Copy".into(),
        ..Default::default()
    };

    let action = editor.build_action();
    assert_eq!(action.name(), "Test Copy");
    assert!(action.enabled());
}

#[test]
fn test_action_editor_build_browser_action() {
    use url_dispatcher::config::Action;
    use url_dispatcher::ui_settings::{ActionEditor, ActionTypeChoice};

    let editor = ActionEditor {
        action_type: ActionTypeChoice::OpenInBrowser,
        name: "Chrome".into(),
        executable_path: "/usr/bin/chrome".into(),
        args_str: "--incognito {URL}".into(),
        ..Default::default()
    };

    let action = editor.build_action();
    assert_eq!(action.name(), "Chrome");
    if let Action::OpenInBrowser {
        executable_path,
        args,
        ..
    } = &action
    {
        assert_eq!(executable_path, "/usr/bin/chrome");
        assert_eq!(args, &["--incognito", "{URL}"]);
    } else {
        panic!("Expected OpenInBrowser");
    }
}

#[test]
fn test_action_editor_build_browser_empty_args() {
    use url_dispatcher::config::Action;
    use url_dispatcher::ui_settings::{ActionEditor, ActionTypeChoice};

    let editor = ActionEditor {
        action_type: ActionTypeChoice::OpenInBrowser,
        name: "Browser".into(),
        executable_path: "/usr/bin/browser".into(),
        args_str: "".into(),
        ..Default::default()
    };

    let action = editor.build_action();
    if let Action::OpenInBrowser { args, .. } = &action {
        // 空参数时应使用默认 {URL} | Empty args should default to {URL}
        assert_eq!(args, &["{URL}"]);
    } else {
        panic!("Expected OpenInBrowser");
    }
}

// ─── get_config_dir 测试 | get_config_dir Tests ─────────────────────────────

#[test]
fn test_get_config_dir_returns_path() {
    let dir = url_dispatcher::config::get_config_dir();
    assert!(dir.is_ok());
    let path = dir.unwrap();
    assert!(path.exists());
    // Linux 下应包含 "url-dispatcher" | On Linux should contain "url-dispatcher"
    assert!(
        path.to_string_lossy().contains("url-dispatcher")
            || path.to_string_lossy().contains("URLDispatcher")
    );
}

#[test]
fn test_get_config_path_returns_json() {
    let path = url_dispatcher::config::get_config_path();
    assert!(path.is_ok());
    let p = path.unwrap();
    assert_eq!(p.file_name().unwrap(), "config.json");
}
