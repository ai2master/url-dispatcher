// ============================================================================
// main.rs — 程序入口 | Application Entry Point
// ============================================================================
//
// 功能概述 | Overview:
//   本模块是 URL Dispatcher 的入口文件。负责解析命令行参数，判断运行模式
//   （分发模式或设置模式），初始化窗口并启动 GUI 事件循环。
//
//   This is the entry point of URL Dispatcher. It parses command-line arguments,
//   determines the run mode (dispatch or settings), initializes the window, and
//   starts the GUI event loop.
//
// 运行模式 | Run Modes:
//   1. 分发模式 (Dispatch Mode):
//      当传入 URL 参数时（如 `url-dispatcher "https://example.com"`），
//      程序以弹窗形式显示可用的操作列表，用户可选择如何处理该 URL。
//      窗口较小（420x350），置顶显示，不可调整大小。
//
//      When a URL argument is provided (e.g. `url-dispatcher "https://example.com"`),
//      the program shows a popup with available actions. The user picks how to
//      handle the URL. The window is small (420x350), always-on-top, non-resizable.
//
//   2. 设置模式 (Settings Mode):
//      不传参数时（直接运行 `url-dispatcher`），打开设置管理界面。
//      窗口较大（650x550），可调整大小。
//
//      When no arguments are given (just `url-dispatcher`), the settings UI opens.
//      The window is larger (650x550) and resizable.
//
// 模块声明 | Module Declarations:
//   - actions:     动作执行（剪贴板、文件追加、浏览器启动）| Action execution
//   - app:         应用状态管理与 eframe 集成 | App state and eframe integration
//   - config:      配置数据结构、JSON 序列化/反序列化 | Config structs and I/O
//   - i18n:        国际化支持（中文/英文双语翻译）| Internationalization (zh-CN / en-US)
//   - platform:    平台集成（注册/取消注册为默认浏览器）| Default browser registration
//   - ui_dispatch: 分发弹窗的 UI 渲染逻辑 | Dispatch popup UI rendering
//   - ui_settings: 设置管理界面的 UI 渲染逻辑 | Settings UI rendering
// ============================================================================

mod actions;
mod app;
mod config;
mod i18n;
mod platform;
mod ui_dispatch;
mod ui_settings;

use app::{App, AppMode};

/// 程序入口函数 | Main entry function
///
/// 执行流程 | Execution flow:
///   1. 收集命令行参数 | Collect CLI arguments
///   2. 判断运行模式：有 URL 参数→分发模式，否则→设置模式
///      Determine mode: URL arg present → dispatch; otherwise → settings
///   3. 加载用户配置（不存在则使用默认值）| Load config (defaults if absent)
///   4. 根据模式设置窗口标题、尺寸和属性 | Configure window per mode
///   5. 启动 eframe 原生窗口事件循环 | Start the eframe native event loop
fn main() -> eframe::Result {
    // ── 步骤 1: 收集命令行参数 | Step 1: Collect CLI arguments ──────────
    let args: Vec<String> = std::env::args().collect();

    // ── 步骤 2: 判断运行模式 | Step 2: Determine run mode ───────────────
    // args[0] 是程序自身路径，args[1]（如果存在）是待处理的 URL
    // args[0] is the program path; args[1] (if present) is the URL to dispatch
    let mode = if args.len() > 1 {
        AppMode::Dispatch(args[1].clone())
    } else {
        AppMode::Settings
    };

    // ── 步骤 3: 加载配置 | Step 3: Load configuration ───────────────────
    // 如果配置文件不存在或解析失败，使用内置默认配置
    // （默认包含"复制到剪贴板"和"追加到文件"两个动作）
    // If config is missing or corrupt, fall back to built-in defaults
    // (includes "Copy to Clipboard" and "Append to File" actions)
    let cfg = config::load_config().unwrap_or_default();

    // ── 步骤 4: 配置窗口参数 | Step 4: Configure window parameters ─────
    // 分发模式：标题含 URL（过长则截断至60字符），窗口小且置顶不可缩放
    // 设置模式：固定标题，窗口较大且可缩放
    // Dispatch: title includes URL (truncated >60 chars), small, on-top, fixed size
    // Settings: fixed title, larger, resizable
    let (title, width, height, is_dispatch) = match &mode {
        AppMode::Dispatch(url) => {
            // 截断过长的 URL，避免标题栏溢出 | Truncate long URLs for the title bar
            let display_url = if url.len() > 60 {
                format!("{}...", &url[..57])
            } else {
                url.clone()
            };
            (
                format!("URL Dispatcher - {}", display_url),
                420.0, // 弹窗宽度 | Popup width
                350.0, // 弹窗高度 | Popup height
                true,
            )
        }
        AppMode::Settings => ("URL Dispatcher - Settings".to_string(), 650.0, 550.0, false),
    };

    // 创建视口（窗口）配置 | Create viewport (window) configuration
    let mut viewport = eframe::egui::ViewportBuilder::default()
        .with_inner_size([width, height])
        .with_resizable(!is_dispatch); // 分发模式固定大小 | Fixed size in dispatch mode

    // 分发模式下窗口置顶，确保弹窗不被其他窗口遮挡
    // In dispatch mode, always-on-top so popup isn't hidden behind other windows
    if is_dispatch {
        viewport = viewport.with_always_on_top();
    }

    // ── 步骤 5: 启动 eframe 原生窗口 | Step 5: Launch eframe native window
    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    // run_native 启动 GUI 事件循环，窗口关闭后才返回
    // run_native starts the GUI event loop; blocks until the window is closed
    eframe::run_native(
        &title,
        options,
        Box::new(move |_cc| Ok(Box::new(App::new(mode, cfg)))),
    )
}
