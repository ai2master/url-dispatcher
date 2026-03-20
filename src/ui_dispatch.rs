/*
 * 分发界面模块 | Dispatcher UI Module
 *
 * 功能概述 | Overview:
 * 本模块实现 URL 分发弹窗的 UI 渲染逻辑。当用户点击 URL 时，
 * 应用启动并显示此界面，让用户选择如何处理 URL。
 *
 * This module implements UI rendering logic for the URL dispatcher popup.
 * When user clicks a URL, the app launches and displays this interface,
 * allowing user to choose how to handle the URL.
 *
 * UI 结构 | UI Structure:
 * ┌─────────────────────────────────────┐
 * │ URL: https://example.com...         │  <- URL 显示区 | URL display area
 * ├─────────────────────────────────────┤
 * │ [1] 复制到剪贴板                      │  <- 动作按钮区 | Action buttons area
 * │ [2] 追加到文件                        │     支持键盘快捷键 1-9
 * │ [3] 在 Firefox 中打开                 │     Supports keyboard shortcuts 1-9
 * ├─────────────────────────────────────┤
 * │ [设置]              [取消]            │  <- 底部栏 | Bottom bar
 * │ 状态消息（成功/错误）                  │     Status message (success/error)
 * └─────────────────────────────────────┘
 *
 * 设计说明 | Design Notes:
 * - 使用 egui 即时模式 GUI 框架
 * - 仅显示已启用的动作
 * - 支持数字键 1-9 快捷键执行动作
 * - 支持 Escape 键关闭窗口
 * - 动作执行成功后自动关闭窗口
 * - 动作执行失败时显示错误消息，不关闭窗口
 *
 * - Uses egui immediate mode GUI framework
 * - Only displays enabled actions
 * - Supports number keys 1-9 as shortcuts to execute actions
 * - Supports Escape key to close window
 * - Automatically closes window after successful action execution
 * - Displays error message and keeps window open on action failure
 */

use eframe::egui;

use crate::actions;
use crate::app::App;
use crate::config::Action;
use crate::i18n::Tr;

impl App {
    /// 渲染分发界面 | Render dispatcher UI
    ///
    /// 在 egui 上下文中渲染 URL 分发界面，包括 URL 显示、动作按钮列表、
    /// 设置/取消按钮和状态消息。
    ///
    /// Renders URL dispatcher UI in egui context, including URL display,
    /// action button list, settings/cancel buttons, and status messages.
    ///
    /// 参数 | Parameters:
    /// - `ctx`: egui 渲染上下文
    ///         egui rendering context
    ///
    /// 布局说明 | Layout Description:
    /// 1. 顶部：显示 URL 标签和 URL 内容（长 URL 会被截断）
    ///    Top: Display URL label and URL content (long URLs are truncated)
    /// 2. 中部：动作按钮区，每个按钮显示序号和动作名称
    ///    Middle: Action buttons area, each button shows number and action name
    /// 3. 底部：设置按钮（左）和取消按钮（右）
    ///    Bottom: Settings button (left) and Cancel button (right)
    /// 4. 状态消息区：显示成功消息（绿色）或错误消息（红色）
    ///    Status message area: Display success message (green) or error message (red)
    ///
    /// 键盘快捷键处理 | Keyboard Shortcut Handling:
    /// - 数字键 1-9: 执行对应序号的动作
    ///              Execute action with corresponding number
    /// - Escape: 关闭窗口
    ///          Close window
    pub fn render_dispatcher_ui(&mut self, ctx: &egui::Context) {
        // 获取当前 URL，如果不在分发模式则直接返回
        // Get current URL, return directly if not in dispatch mode
        let url = match &self.mode {
            crate::app::AppMode::Dispatch(u) => u.clone(),
            _ => return,
        };
        let lang = self.config.language;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.add_space(8.0);

                // ─── URL 显示区 | URL Display Area ───
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(Tr::url_label(lang)).strong());
                });
                ui.add_space(4.0);

                // URL 显示框，使用分组框架美化
                // URL display box, using grouped frame for beautification
                egui::Frame::group(ui.style())
                    .inner_margin(8.0)
                    .show(ui, |ui| {
                        ui.set_max_width(ui.available_width());
                        // 长 URL 截断显示，避免界面过宽
                        // Truncate long URLs to avoid overly wide interface
                        let url_text = if url.len() > 120 {
                            format!("{}...", &url[..117])
                        } else {
                            url.clone()
                        };
                        ui.label(
                            egui::RichText::new(&url_text)
                                .monospace()  // 等宽字体便于阅读 URL | Monospace font for URL readability
                                .size(13.0),
                        );
                    });

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(8.0);

                // ─── 动作按钮区 | Action Buttons Area ───

                // 过滤出已启用的动作
                // Filter enabled actions
                let enabled_actions: Vec<(usize, Action)> = self
                    .config
                    .actions
                    .iter()
                    .enumerate()
                    .filter(|(_, a)| a.enabled())
                    .map(|(i, a)| (i, a.clone()))
                    .collect();

                // 记录用户选择的动作（点击或快捷键）
                // Record action selected by user (click or shortcut)
                let mut action_to_execute: Option<Action> = None;

                // 渲染每个动作按钮
                // Render each action button
                for (display_idx, (_config_idx, action)) in enabled_actions.iter().enumerate() {
                    let shortcut_num = display_idx + 1;
                    let label = format!("[{}] {}", shortcut_num, action.name());

                    let button = egui::Button::new(
                        egui::RichText::new(&label).size(15.0),
                    )
                    .min_size(egui::vec2(ui.available_width(), 32.0));

                    // 检测鼠标点击 | Detect mouse click
                    if ui.add(button).clicked() {
                        action_to_execute = Some(action.clone());
                    }

                    // 检测键盘快捷键 1-9 | Detect keyboard shortcuts 1-9
                    if shortcut_num <= 9 {
                        let key = match shortcut_num {
                            1 => egui::Key::Num1,
                            2 => egui::Key::Num2,
                            3 => egui::Key::Num3,
                            4 => egui::Key::Num4,
                            5 => egui::Key::Num5,
                            6 => egui::Key::Num6,
                            7 => egui::Key::Num7,
                            8 => egui::Key::Num8,
                            9 => egui::Key::Num9,
                            _ => unreachable!(),
                        };
                        if ctx.input(|i| i.key_pressed(key)) {
                            action_to_execute = Some(action.clone());
                        }
                    }
                }

                // 检测 Escape 键关闭窗口 | Detect Escape key to close window
                if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                    self.should_close = true;
                }

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(4.0);

                // ─── 底部栏：设置 + 取消 | Bottom Bar: Settings + Cancel ───
                ui.horizontal(|ui| {
                    // 设置按钮（左侧）| Settings button (left)
                    if ui.button(Tr::settings(lang)).clicked() {
                        self.mode = crate::app::AppMode::Settings;
                    }
                    // 取消按钮（右侧）| Cancel button (right)
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(Tr::cancel(lang)).clicked() {
                            self.should_close = true;
                        }
                    });
                });

                // ─── 状态消息区 | Status Message Area ───
                if let Some(msg) = &self.status_message {
                    ui.add_space(8.0);
                    // 根据是否为错误选择颜色
                    // Choose color based on whether it's an error
                    let color = if self.status_is_error {
                        egui::Color32::RED
                    } else {
                        egui::Color32::GREEN
                    };
                    ui.label(egui::RichText::new(msg).color(color));
                }

                // 执行用户选择的动作（如果有）
                // Execute action selected by user (if any)
                if let Some(action) = action_to_execute {
                    self.execute_action(&action, &url);
                }
            });
        });
    }

    /// 执行动作 | Execute action
    ///
    /// 根据动作类型调用相应的执行函数，并处理执行结果。
    /// 成功时关闭窗口，失败时显示错误消息。
    ///
    /// Calls corresponding execution function based on action type,
    /// and handles execution result. Closes window on success,
    /// displays error message on failure.
    ///
    /// 参数 | Parameters:
    /// - `action`: 要执行的动作
    ///            Action to execute
    /// - `url`: 要处理的 URL
    ///         URL to process
    ///
    /// 分发逻辑 | Dispatch Logic:
    /// - CopyToClipboard: 调用 actions::copy_to_clipboard
    ///                   Call actions::copy_to_clipboard
    /// - AppendToFile: 检查路径配置，调用 actions::append_to_file
    ///                Check path configuration, call actions::append_to_file
    /// - OpenInBrowser: 调用 actions::open_in_browser，传递可执行文件和参数
    ///                 Call actions::open_in_browser with executable and arguments
    ///
    /// 错误处理 | Error Handling:
    /// - 成功：设置 should_close 为 true，窗口将在下一帧关闭
    ///        Success: Set should_close to true, window will close on next frame
    /// - 失败：设置 status_message 和 status_is_error，显示错误消息
    ///        Failure: Set status_message and status_is_error, display error message
    fn execute_action(&mut self, action: &Action, url: &str) {
        let lang = self.config.language;

        // 根据动作类型执行相应操作
        // Execute corresponding operation based on action type
        let result = match action {
            Action::CopyToClipboard { .. } => actions::copy_to_clipboard(url),
            Action::AppendToFile { .. } => {
                // 检查追加文件路径是否已配置
                // Check if append file path is configured
                if let Some(path) = &self.config.append_file_path {
                    actions::append_to_file(url, path)
                } else {
                    Err(anyhow::anyhow!("{}", Tr::append_path_not_configured(lang)))
                }
            }
            Action::OpenInBrowser {
                executable_path,
                args,
                ..
            } => actions::open_in_browser(url, executable_path, args),
        };

        // 处理执行结果 | Handle execution result
        match result {
            Ok(_) => {
                // 成功：标记窗口应关闭
                // Success: Mark window should close
                self.should_close = true;
            }
            Err(e) => {
                // 失败：显示错误消息，保持窗口打开
                // Failure: Display error message, keep window open
                self.status_message = Some(Tr::error_prefix(lang, &e.to_string()));
                self.status_is_error = true;
            }
        }
    }
}
