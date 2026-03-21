/*
 * 设置界面模块 | Settings UI Module
 *
 * 功能概述 | Overview:
 * 本模块实现设置界面的 UI 渲染逻辑，包括动作管理、追加文件路径配置、
 * 系统集成（注册为默认浏览器）和语言选择。
 *
 * This module implements UI rendering logic for settings interface, including
 * action management, append file path configuration, system integration
 * (register as default browser), and language selection.
 *
 * UI 结构 | UI Structure:
 * ┌─────────────────────────────────────────────────┐
 * │ URL Dispatcher 设置         [语言选择器]         │  <- 标题栏 | Title bar
 * ├─────────────────────────────────────────────────┤
 * │ ┌─ 动作列表 ────────────────────────────────┐  │
 * │ │ ☑ 复制到剪贴板 (Copy) [上移][下移][编辑][删除] │  │  <- 动作管理区
 * │ │ ☑ 追加到文件 (Append)   [上移][下移][编辑][删除] │  │     Action mgmt
 * │ │ [+ 添加动作]                               │  │
 * │ └───────────────────────────────────────────┘  │
 * │ ┌─ 追加文件路径 ─────────────────────────────┐  │
 * │ │ [文本框: /path/to/file.txt]               │  │  <- 文件路径配置
 * │ └───────────────────────────────────────────┘  │     File path config
 * │ ┌─ 系统集成 ────────────────────────────────┐  │
 * │ │ [注册为默认浏览器] [取消注册]              │  │  <- 系统集成
 * │ └───────────────────────────────────────────┘  │     System integration
 * │ [保存配置] (config.json路径)                   │  <- 底部栏 | Bottom bar
 * │ 状态消息                                       │
 * └─────────────────────────────────────────────────┘
 *
 * 设计说明 | Design Notes:
 * - 使用 egui 即时模式 GUI 框架
 * - 动作列表支持启用/禁用、上移/下移、编辑、删除
 * - 动作编辑器使用独立弹窗
 * - 所有修改在内存中进行，点击"保存配置"才写入磁盘
 * - 注册/取消注册操作立即生效，无需保存配置
 *
 * - Uses egui immediate mode GUI framework
 * - Action list supports enable/disable, move up/down, edit, delete
 * - Action editor uses independent popup window
 * - All modifications are in-memory, only written to disk when "Save Configuration" clicked
 * - Register/unregister operations take effect immediately, no need to save configuration
 */

use eframe::egui;
use uuid::Uuid;

use crate::app::App;
use crate::config::{self, save_config, Action};
use crate::i18n::{Language, Tr};
use crate::platform;

/// 动作类型选择枚举 | Action Type Choice Enum
///
/// 在动作编辑器中用于选择要创建或编辑的动作类型。
/// Used in action editor to select the type of action to create or edit.
///
/// 变体说明 | Variants:
/// - `CopyToClipboard`: 复制到剪贴板
///                     Copy to clipboard
/// - `AppendToFile`: 追加到文件
///                  Append to file
/// - `OpenInBrowser`: 在浏览器中打开
///                   Open in browser
#[derive(Debug, Clone, PartialEq)]
pub enum ActionTypeChoice {
    CopyToClipboard,
    AppendToFile,
    OpenInBrowser,
}

impl ActionTypeChoice {
    /// 获取动作类型的本地化标签 | Get localized label for action type
    fn label(&self, lang: Language) -> &'static str {
        match self {
            ActionTypeChoice::CopyToClipboard => Tr::copy_to_clipboard(lang),
            ActionTypeChoice::AppendToFile => Tr::append_to_file(lang),
            ActionTypeChoice::OpenInBrowser => Tr::open_in_browser(lang),
        }
    }
}

/// 动作编辑器状态 | Action Editor State
///
/// 管理动作编辑弹窗的状态，包括是否激活、编辑模式（新建/编辑）、
/// 以及表单字段的当前值。
///
/// Manages state of action editor popup, including activation status,
/// edit mode (create/edit), and current values of form fields.
///
/// 字段说明 | Fields:
/// - `active`: 编辑器是否激活（显示）
///            Whether editor is active (visible)
/// - `editing_id`: 正在编辑的动作 ID（None 表示新建模式）
///                ID of action being edited (None means create mode)
/// - `action_type`: 选择的动作类型
///                 Selected action type
/// - `name`: 动作名称（用户输入）
///          Action name (user input)
/// - `executable_path`: 可执行文件路径（仅 OpenInBrowser 类型）
///                     Executable file path (OpenInBrowser type only)
/// - `args_str`: 命令行参数字符串（仅 OpenInBrowser 类型）
///              Command line arguments string (OpenInBrowser type only)
pub struct ActionEditor {
    pub active: bool,
    pub editing_id: Option<Uuid>,
    pub action_type: ActionTypeChoice,
    pub name: String,
    pub executable_path: String,
    pub args_str: String,
}

impl Default for ActionEditor {
    fn default() -> Self {
        Self {
            active: false,
            editing_id: None,
            action_type: ActionTypeChoice::OpenInBrowser,
            name: String::new(),
            executable_path: String::new(),
            args_str: "{URL}".into(), // 默认参数模板 | Default argument template
        }
    }
}

impl ActionEditor {
    /// 打开编辑器以创建新动作 | Open editor to create new action
    ///
    /// 重置所有字段为默认值，设置为新建模式。
    /// Resets all fields to default values, sets to create mode.
    pub fn open_new(&mut self) {
        *self = Self {
            active: true,
            ..Default::default()
        };
    }

    /// 打开编辑器以编辑已存在的动作 | Open editor to edit existing action
    ///
    /// 从给定动作加载字段值，设置为编辑模式。
    /// Loads field values from given action, sets to edit mode.
    ///
    /// 参数 | Parameters:
    /// - `action`: 要编辑的动作
    ///            Action to edit
    pub fn open_edit(&mut self, action: &Action) {
        self.active = true;
        self.editing_id = Some(action.id());

        // 根据动作类型设置字段值 | Set field values based on action type
        match action {
            Action::CopyToClipboard { name, .. } => {
                self.action_type = ActionTypeChoice::CopyToClipboard;
                self.name = name.clone();
                self.executable_path.clear();
                self.args_str.clear();
            }
            Action::AppendToFile { name, .. } => {
                self.action_type = ActionTypeChoice::AppendToFile;
                self.name = name.clone();
                self.executable_path.clear();
                self.args_str.clear();
            }
            Action::OpenInBrowser {
                name,
                executable_path,
                args,
                ..
            } => {
                self.action_type = ActionTypeChoice::OpenInBrowser;
                self.name = name.clone();
                self.executable_path = executable_path.clone();
                // 将参数数组连接为字符串 | Join argument array into string
                self.args_str = args.join(" ");
            }
        }
    }

    /// 构建动作对象 | Build action object
    ///
    /// 根据当前字段值构建 Action 枚举实例。在编辑模式下使用现有 ID，
    /// 在新建模式下生成新 ID。
    ///
    /// Builds Action enum instance from current field values. Uses existing ID
    /// in edit mode, generates new ID in create mode.
    ///
    /// 返回值 | Return:
    /// 构建的动作对象。
    /// Built action object.
    pub fn build_action(&self) -> Action {
        // 使用现有 ID 或生成新 ID | Use existing ID or generate new ID
        let id = self.editing_id.unwrap_or_else(Uuid::new_v4);

        // 根据动作类型构建相应的枚举变体 | Build corresponding enum variant based on action type
        match self.action_type {
            ActionTypeChoice::CopyToClipboard => Action::CopyToClipboard {
                id,
                name: self.name.clone(),
                enabled: true,
            },
            ActionTypeChoice::AppendToFile => Action::AppendToFile {
                id,
                name: self.name.clone(),
                enabled: true,
            },
            ActionTypeChoice::OpenInBrowser => {
                // 解析参数字符串为数组 | Parse argument string into array
                let args: Vec<String> = if self.args_str.trim().is_empty() {
                    vec!["{URL}".into()] // 空参数时使用默认 | Use default when empty
                } else {
                    shell_words_parse(&self.args_str)
                };
                Action::OpenInBrowser {
                    id,
                    name: self.name.clone(),
                    enabled: true,
                    executable_path: self.executable_path.clone(),
                    args,
                }
            }
        }
    }
}

/// 简单的参数解析器，处理带引号的字符串 | Simple argument parser that handles quoted strings
///
/// 将命令行参数字符串解析为参数数组，支持单引号和双引号。
/// 引号内的空格不会分割参数。
///
/// Parses command line argument string into argument array, supports single
/// and double quotes. Spaces within quotes won't split arguments.
///
/// 参数 | Parameters:
/// - `s`: 要解析的参数字符串
///       Argument string to parse
///
/// 返回值 | Return:
/// 解析后的参数数组。
/// Parsed argument array.
///
/// 解析逻辑 | Parsing Logic:
/// 1. 遍历每个字符
///    Iterate through each character
/// 2. 遇到引号（" 或 '）进入引号模式，记录引号字符
///    Enter quote mode when encountering quote (" or '), record quote character
/// 3. 在引号模式中，只有匹配的引号可以退出，其他字符加入当前参数
///    In quote mode, only matching quote can exit, other characters join current argument
/// 4. 不在引号模式中，空格分割参数
///    Outside quote mode, spaces split arguments
/// 5. 完成遍历后，如果有未完成的参数，加入结果
///    After iteration, if there's incomplete argument, add to result
///
/// 示例 | Examples:
/// - `--incognito {URL}` -> ["--incognito", "{URL}"]
/// - `--new-window "{URL}"` -> ["--new-window", "{URL}"]
/// - `--arg1 'value with spaces' --arg2` -> ["--arg1", "value with spaces", "--arg2"]
fn shell_words_parse(s: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_quote = false;
    let mut quote_char = ' ';

    for ch in s.chars() {
        if in_quote {
            // 在引号模式中 | In quote mode
            if ch == quote_char {
                // 匹配的引号，退出引号模式 | Matching quote, exit quote mode
                in_quote = false;
            } else {
                // 其他字符加入当前参数 | Other characters join current argument
                current.push(ch);
            }
        } else if ch == '"' || ch == '\'' {
            // 遇到引号，进入引号模式 | Encounter quote, enter quote mode
            in_quote = true;
            quote_char = ch;
        } else if ch == ' ' {
            // 空格分割参数 | Space splits arguments
            if !current.is_empty() {
                args.push(current.clone());
                current.clear();
            }
        } else {
            // 普通字符加入当前参数 | Normal character joins current argument
            current.push(ch);
        }
    }
    // 处理最后一个参数 | Handle last argument
    if !current.is_empty() {
        args.push(current);
    }
    args
}

impl App {
    /// 渲染设置界面 | Render settings UI
    ///
    /// 在 egui 上下文中渲染设置界面，包括动作管理、文件路径配置、
    /// 系统集成和语言选择。
    ///
    /// Renders settings UI in egui context, including action management,
    /// file path configuration, system integration, and language selection.
    ///
    /// 参数 | Parameters:
    /// - `ctx`: egui 渲染上下文
    ///         egui rendering context
    ///
    /// 各区域说明 | Section Description:
    /// 1. 标题栏：显示标题和语言选择器
    ///    Title bar: Display title and language selector
    /// 2. 动作列表：显示所有动作，支持启用/禁用、上移/下移、编辑、删除
    ///    Action list: Display all actions, support enable/disable, move up/down, edit, delete
    /// 3. 追加文件路径：配置"追加到文件"动作的目标文件
    ///    Append file path: Configure target file for "Append to File" action
    /// 4. 系统集成：注册/取消注册为默认浏览器
    ///    System integration: Register/unregister as default browser
    /// 5. 保存配置：将当前配置写入磁盘
    ///    Save configuration: Write current configuration to disk
    pub fn render_settings_ui(&mut self, ctx: &egui::Context) {
        let lang = self.config.language;

        egui::CentralPanel::default().show(ctx, |ui| {
            // ─── 标题栏：标题 + 语言选择器 | Title Bar: Title + Language Selector ───
            ui.horizontal(|ui| {
                ui.heading(Tr::settings_title(lang));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // 保存之前的语言，用于显示标签
                    // Save previous language for displaying label
                    let prev_lang = self.config.language;
                    egui::ComboBox::from_id_salt("lang_combo")
                        .selected_text(self.config.language.label())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.config.language,
                                Language::English,
                                Language::English.label(),
                            );
                            ui.selectable_value(
                                &mut self.config.language,
                                Language::Chinese,
                                Language::Chinese.label(),
                            );
                        });
                    ui.label(Tr::language_label(prev_lang));
                });
            });
            ui.add_space(8.0);

            // 使用可能已更新的语言 | Use potentially updated language
            let lang = self.config.language;

            // ═══════════════════════════════════════════════════════════
            // 动作列表区 | Actions List Section
            // ═══════════════════════════════════════════════════════════
            ui.group(|ui| {
                ui.set_min_width(ui.available_width());
                ui.label(egui::RichText::new(Tr::actions(lang)).strong().size(16.0));
                ui.add_space(4.0);

                // 记录用户操作（由于 egui 即时模式，不能在迭代中修改）
                // Record user operations (can't modify during iteration due to egui immediate mode)
                let mut toggle_idx: Option<(usize, bool)> = None;
                let mut delete_idx: Option<usize> = None;
                let mut edit_action: Option<Action> = None;
                let mut move_up_idx: Option<usize> = None;
                let mut move_down_idx: Option<usize> = None;

                let action_count = self.config.actions.len();
                // 渲染每个动作行 | Render each action row
                for (i, action) in self.config.actions.iter().enumerate() {
                    ui.horizontal(|ui| {
                        let mut enabled = action.enabled();
                        if ui.checkbox(&mut enabled, "").changed() {
                            toggle_idx = Some((i, enabled));
                        }

                        ui.label(egui::RichText::new(action.name()).size(14.0));
                        ui.label(
                            egui::RichText::new(format!("({})", action.type_label(lang)))
                                .weak()
                                .size(12.0),
                        );

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.small_button(Tr::delete(lang)).clicked() {
                                delete_idx = Some(i);
                            }
                            if ui.small_button(Tr::edit(lang)).clicked() {
                                edit_action = Some(action.clone());
                            }
                            if i + 1 < action_count && ui.small_button(Tr::down(lang)).clicked() {
                                move_down_idx = Some(i);
                            }
                            if i > 0 && ui.small_button(Tr::up(lang)).clicked() {
                                move_up_idx = Some(i);
                            }
                        });
                    });
                    ui.separator();
                }

                // 应用用户操作（在迭代结束后）| Apply user operations (after iteration)
                if let Some((idx, val)) = toggle_idx {
                    self.config.actions[idx].set_enabled(val);
                }
                if let Some(idx) = delete_idx {
                    self.config.actions.remove(idx);
                }
                if let Some(action) = edit_action {
                    self.action_editor.open_edit(&action);
                }
                if let Some(idx) = move_up_idx {
                    // 与前一个动作交换位置 | Swap with previous action
                    self.config.actions.swap(idx, idx - 1);
                }
                if let Some(idx) = move_down_idx {
                    // 与后一个动作交换位置 | Swap with next action
                    self.config.actions.swap(idx, idx + 1);
                }

                ui.add_space(4.0);
                // 添加新动作按钮 | Add new action button
                if ui.button(Tr::add_action(lang)).clicked() {
                    self.action_editor.open_new();
                }
            });

            ui.add_space(12.0);

            // ═══════════════════════════════════════════════════════════
            // 追加文件路径配置区 | Append File Path Configuration Section
            // ═══════════════════════════════════════════════════════════
            ui.group(|ui| {
                ui.set_min_width(ui.available_width());
                ui.label(
                    egui::RichText::new(Tr::append_file_path(lang))
                        .strong()
                        .size(16.0),
                );
                ui.add_space(4.0);
                ui.label(Tr::append_file_description(lang));

                let mut path_str = self
                    .config
                    .append_file_path
                    .as_ref()
                    .map(|p| p.display().to_string())
                    .unwrap_or_default();

                if ui.text_edit_singleline(&mut path_str).changed() {
                    if path_str.is_empty() {
                        self.config.append_file_path = None;
                    } else {
                        self.config.append_file_path = Some(std::path::PathBuf::from(&path_str));
                    }
                }
            });

            ui.add_space(12.0);

            // ═══════════════════════════════════════════════════════════
            // 系统集成区 | System Integration Section
            // ═══════════════════════════════════════════════════════════
            ui.group(|ui| {
                ui.set_min_width(ui.available_width());
                ui.label(
                    egui::RichText::new(Tr::system_integration(lang))
                        .strong()
                        .size(16.0),
                );
                ui.add_space(4.0);

                ui.horizontal(|ui| {
                    if ui.button(Tr::register_default_browser(lang)).clicked() {
                        match std::env::current_exe() {
                            Ok(exe) => match platform::register_as_default_browser(&exe) {
                                Ok(_) => {
                                    self.status_message = Some(Tr::registered_ok(lang).into());
                                    self.status_is_error = false;
                                }
                                Err(e) => {
                                    self.status_message =
                                        Some(Tr::register_failed(lang, &e.to_string()));
                                    self.status_is_error = true;
                                }
                            },
                            Err(e) => {
                                self.status_message =
                                    Some(Tr::exe_path_error(lang, &e.to_string()));
                                self.status_is_error = true;
                            }
                        }
                    }

                    if ui.button(Tr::unregister(lang)).clicked() {
                        match platform::unregister_as_default_browser() {
                            Ok(_) => {
                                self.status_message = Some(Tr::unregistered_ok(lang).into());
                                self.status_is_error = false;
                            }
                            Err(e) => {
                                self.status_message =
                                    Some(Tr::unregister_failed(lang, &e.to_string()));
                                self.status_is_error = true;
                            }
                        }
                    }
                });

                #[cfg(windows)]
                {
                    ui.add_space(4.0);
                    ui.label(egui::RichText::new(Tr::windows_hint(lang)).weak());
                }
            });

            ui.add_space(12.0);

            // ═══════════════════════════════════════════════════════════
            // 保存配置区 / 状态消息区 | Save Configuration / Status Message Section
            // ═══════════════════════════════════════════════════════════
            ui.horizontal(|ui| {
                if ui
                    .button(egui::RichText::new(Tr::save_configuration(lang)).size(15.0))
                    .clicked()
                {
                    match save_config(&self.config) {
                        Ok(_) => {
                            self.status_message = Some(Tr::config_saved(lang).into());
                            self.status_is_error = false;
                        }
                        Err(e) => {
                            self.status_message = Some(Tr::save_failed(lang, &e.to_string()));
                            self.status_is_error = true;
                        }
                    }
                }

                // Show config file location
                if let Ok(path) = config::get_config_path() {
                    ui.label(
                        egui::RichText::new(format!("({})", path.display()))
                            .weak()
                            .size(11.0),
                    );
                }
            });

            if let Some(msg) = &self.status_message {
                ui.add_space(8.0);
                let color = if self.status_is_error {
                    egui::Color32::RED
                } else {
                    egui::Color32::GREEN
                };
                ui.label(egui::RichText::new(msg).color(color));
            }
        });

        // ═══════════════════════════════════════════════════════════
        // 动作编辑器弹窗 | Action Editor Popup
        // ═══════════════════════════════════════════════════════════
        if self.action_editor.active {
            self.render_action_editor(ctx);
        }
    }

    /// 渲染动作编辑器弹窗 | Render action editor popup
    ///
    /// 显示独立的弹窗用于创建或编辑动作。根据 editing_id 是否为 None
    /// 判断是新建模式还是编辑模式。
    ///
    /// Displays independent popup for creating or editing action. Determines
    /// whether it's create mode or edit mode based on whether editing_id is None.
    ///
    /// 参数 | Parameters:
    /// - `ctx`: egui 渲染上下文
    ///         egui rendering context
    ///
    /// 编辑器逻辑 | Editor Logic:
    /// 1. 显示动作类型下拉框（复制/追加/浏览器）
    ///    Display action type dropdown (copy/append/browser)
    /// 2. 显示名称输入框
    ///    Display name input field
    /// 3. 如果是浏览器类型，显示可执行文件路径和参数输入框
    ///    If browser type, display executable path and arguments input fields
    /// 4. 保存按钮：验证必填字段，构建动作并添加到配置
    ///    Save button: Validate required fields, build action and add to configuration
    /// 5. 取消按钮：关闭编辑器
    ///    Cancel button: Close editor
    fn render_action_editor(&mut self, ctx: &egui::Context) {
        let lang = self.config.language;
        let mut open = self.action_editor.active;

        // 根据模式选择窗口标题 | Choose window title based on mode
        egui::Window::new(if self.action_editor.editing_id.is_some() {
            Tr::edit_action(lang)
        } else {
            Tr::add_action_title(lang)
        })
        .open(&mut open)
        .resizable(false)
        .collapsible(false)
        .min_width(350.0)
        .show(ctx, |ui| {
            // ─── 动作类型选择 | Action Type Selection ───
            ui.horizontal(|ui| {
                ui.label(Tr::type_label(lang));
                egui::ComboBox::from_id_salt("action_type_combo")
                    .selected_text(self.action_editor.action_type.label(lang))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.action_editor.action_type,
                            ActionTypeChoice::CopyToClipboard,
                            Tr::copy_to_clipboard(lang),
                        );
                        ui.selectable_value(
                            &mut self.action_editor.action_type,
                            ActionTypeChoice::AppendToFile,
                            Tr::append_to_file(lang),
                        );
                        ui.selectable_value(
                            &mut self.action_editor.action_type,
                            ActionTypeChoice::OpenInBrowser,
                            Tr::open_in_browser(lang),
                        );
                    });
            });

            ui.add_space(4.0);

            // ─── 动作名称输入 | Action Name Input ───
            ui.horizontal(|ui| {
                ui.label(Tr::name_label(lang));
                ui.text_edit_singleline(&mut self.action_editor.name);
            });

            // ─── 浏览器类型专用字段 | Browser Type Specific Fields ───
            if self.action_editor.action_type == ActionTypeChoice::OpenInBrowser {
                ui.add_space(4.0);
                // 可执行文件路径 | Executable file path
                ui.horizontal(|ui| {
                    ui.label(Tr::executable_label(lang));
                    ui.text_edit_singleline(&mut self.action_editor.executable_path);
                });

                ui.add_space(4.0);
                // 命令行参数 | Command line arguments
                ui.horizontal(|ui| {
                    ui.label(Tr::arguments_label(lang));
                    ui.text_edit_singleline(&mut self.action_editor.args_str);
                });
                // 参数提示 | Arguments hint
                ui.label(egui::RichText::new(Tr::args_hint(lang)).weak().size(11.0));
            }

            ui.add_space(8.0);

            // ─── 保存 / 取消按钮 | Save / Cancel Buttons ───
            ui.horizontal(|ui| {
                // 验证必填字段 | Validate required fields
                let name_empty = self.action_editor.name.trim().is_empty();
                let exe_empty = self.action_editor.action_type == ActionTypeChoice::OpenInBrowser
                    && self.action_editor.executable_path.trim().is_empty();

                let can_save = !name_empty && !exe_empty;

                // 保存按钮 | Save button
                if ui
                    .add_enabled(can_save, egui::Button::new(Tr::save(lang)))
                    .clicked()
                {
                    let new_action = self.action_editor.build_action();
                    if let Some(edit_id) = self.action_editor.editing_id {
                        // 编辑模式：替换现有动作 | Edit mode: Replace existing action
                        if let Some(pos) =
                            self.config.actions.iter().position(|a| a.id() == edit_id)
                        {
                            // 保留原有的启用状态 | Preserve original enabled status
                            let was_enabled = self.config.actions[pos].enabled();
                            let mut action = new_action;
                            action.set_enabled(was_enabled);
                            self.config.actions[pos] = action;
                        }
                    } else {
                        // 新建模式：添加到列表末尾 | Create mode: Add to end of list
                        self.config.actions.push(new_action);
                    }
                    self.action_editor.active = false;
                }

                // 取消按钮 | Cancel button
                if ui.button(Tr::cancel(lang)).clicked() {
                    self.action_editor.active = false;
                }
            });
        });

        // 更新编辑器激活状态 | Update editor active status
        self.action_editor.active = open;
    }
}
