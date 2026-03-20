// ============================================================================
// app.rs — 应用主状态管理和 eframe 集成 | App State Management & eframe Integration
// ============================================================================
//
// 功能概述 | Overview:
//   定义应用的核心状态结构体 `App` 和运行模式枚举 `AppMode`。
//   实现 eframe::App trait，作为 GUI 事件循环的入口，在每帧（frame）
//   根据当前模式调度到不同的 UI 渲染函数。
//
//   Defines the core state struct `App` and the `AppMode` enum.
//   Implements the `eframe::App` trait, which serves as the entry point for
//   the GUI event loop. Each frame, it dispatches to the appropriate UI
//   renderer based on the current mode.
//
// 设计说明 | Design Notes:
//   - egui 使用"即时模式"(Immediate Mode) GUI 范式：每帧重新绘制整个界面
//     egui uses an Immediate Mode GUI paradigm: the entire UI is redrawn each frame
//   - `should_close` 标志用于从子模块（如动作执行成功后）请求关闭窗口
//     The `should_close` flag lets sub-modules request window closure
//   - `status_message` 用于在 UI 底部显示操作结果（成功/失败）
//     `status_message` shows operation results (success/error) at the UI bottom
// ============================================================================

use eframe::egui;

use crate::config::Config;
use crate::ui_settings::ActionEditor;

/// 应用模式枚举 | Application mode enum
///
/// 决定应用的 UI 和行为：
/// Determines the application's UI and behavior:
///
/// - `Dispatch(url)`: 分发模式 — 显示弹窗，让用户选择如何处理传入的 URL
///   Dispatch mode — shows a popup for the user to choose how to handle the URL
///
/// - `Settings`: 设置模式 — 显示完整的配置管理界面
///   Settings mode — shows the full configuration management UI
#[derive(Debug, Clone)]
pub enum AppMode {
    /// 分发模式，包含待处理的 URL 字符串
    /// Dispatch mode, containing the URL string to handle
    Dispatch(String),
    /// 设置模式，无附加数据
    /// Settings mode, no additional data
    Settings,
}

/// 应用主状态结构体 | Main application state struct
///
/// 字段说明 | Field descriptions:
/// - `mode`:           当前运行模式（分发或设置）| Current run mode (dispatch or settings)
/// - `config`:         用户配置（动作列表、文件路径、语言等）| User config (actions, paths, lang)
/// - `status_message`: 底部状态栏消息（操作成功/失败的提示）| Status bar message (success/error)
/// - `status_is_error`: 状态消息是否为错误（控制颜色：红/绿）| Is the message an error (red/green)
/// - `should_close`:   是否请求关闭窗口（动作执行成功后设为 true）| Request window close
/// - `action_editor`:  动作编辑器状态（新增/编辑动作的弹窗）| Action editor state (add/edit dialog)
pub struct App {
    pub mode: AppMode,
    pub config: Config,
    pub status_message: Option<String>,
    pub status_is_error: bool,
    pub should_close: bool,
    pub action_editor: ActionEditor,
}

impl App {
    /// 创建新的 App 实例 | Create a new App instance
    ///
    /// 参数 | Parameters:
    /// - `mode`:   运行模式（从命令行参数解析得到）| Run mode (from CLI args)
    /// - `config`: 已加载的用户配置 | Loaded user config
    pub fn new(mode: AppMode, config: Config) -> Self {
        Self {
            mode,
            config,
            status_message: None,
            status_is_error: false,
            should_close: false,
            action_editor: ActionEditor::default(),
        }
    }
}

/// eframe::App trait 实现 | eframe::App trait implementation
///
/// `update` 方法在每个 GUI 帧被调用。流程：
/// The `update` method is called every GUI frame. Flow:
///   1. 检查 should_close 标志，如为 true 则发送关闭命令
///      Check should_close; if true, send close viewport command
///   2. 根据 mode 选择渲染分发 UI 或设置 UI
///      Based on mode, render either the dispatch UI or settings UI
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 如果收到关闭请求（如动作执行成功后），立即关闭窗口
        // If close was requested (e.g. after successful action), close immediately
        if self.should_close {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }

        // 根据模式调度到对应的 UI 渲染方法
        // Dispatch to the appropriate UI renderer based on mode
        match self.mode.clone() {
            AppMode::Dispatch(_) => {
                self.render_dispatcher_ui(ctx);
            }
            AppMode::Settings => {
                self.render_settings_ui(ctx);
            }
        }
    }
}
