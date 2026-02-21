use eframe::egui;
use uuid::Uuid;

use crate::app::App;
use crate::config::{self, Action, save_config};
use crate::platform;

#[derive(Debug, Clone, PartialEq)]
pub enum ActionTypeChoice {
    CopyToClipboard,
    AppendToFile,
    OpenInBrowser,
}

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
            args_str: "{URL}".into(),
        }
    }
}

impl ActionEditor {
    pub fn open_new(&mut self) {
        *self = Self {
            active: true,
            ..Default::default()
        };
    }

    pub fn open_edit(&mut self, action: &Action) {
        self.active = true;
        self.editing_id = Some(action.id());
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
                self.args_str = args.join(" ");
            }
        }
    }

    pub fn build_action(&self) -> Action {
        let id = self.editing_id.unwrap_or_else(Uuid::new_v4);
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
                let args: Vec<String> = if self.args_str.trim().is_empty() {
                    vec!["{URL}".into()]
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

/// Simple argument parser that handles quoted strings
fn shell_words_parse(s: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_quote = false;
    let mut quote_char = ' ';

    for ch in s.chars() {
        if in_quote {
            if ch == quote_char {
                in_quote = false;
            } else {
                current.push(ch);
            }
        } else if ch == '"' || ch == '\'' {
            in_quote = true;
            quote_char = ch;
        } else if ch == ' ' {
            if !current.is_empty() {
                args.push(current.clone());
                current.clear();
            }
        } else {
            current.push(ch);
        }
    }
    if !current.is_empty() {
        args.push(current);
    }
    args
}

impl App {
    pub fn render_settings_ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("URL Dispatcher Settings");
            ui.add_space(8.0);

            // ── Actions list ─────────────────────────────────────────
            ui.group(|ui| {
                ui.set_min_width(ui.available_width());
                ui.label(egui::RichText::new("Actions").strong().size(16.0));
                ui.add_space(4.0);

                let mut toggle_idx: Option<(usize, bool)> = None;
                let mut delete_idx: Option<usize> = None;
                let mut edit_action: Option<Action> = None;
                let mut move_up_idx: Option<usize> = None;
                let mut move_down_idx: Option<usize> = None;

                let action_count = self.config.actions.len();
                for (i, action) in self.config.actions.iter().enumerate() {
                    ui.horizontal(|ui| {
                        let mut enabled = action.enabled();
                        if ui.checkbox(&mut enabled, "").changed() {
                            toggle_idx = Some((i, enabled));
                        }

                        ui.label(
                            egui::RichText::new(action.name()).size(14.0),
                        );
                        ui.label(
                            egui::RichText::new(format!("({})", action.type_label()))
                                .weak()
                                .size(12.0),
                        );

                        ui.with_layout(
                            egui::Layout::right_to_left(egui::Align::Center),
                            |ui| {
                                if ui.small_button("Delete").clicked() {
                                    delete_idx = Some(i);
                                }
                                if ui.small_button("Edit").clicked() {
                                    edit_action = Some(action.clone());
                                }
                                if i + 1 < action_count {
                                    if ui.small_button("Down").clicked() {
                                        move_down_idx = Some(i);
                                    }
                                }
                                if i > 0 {
                                    if ui.small_button("Up").clicked() {
                                        move_up_idx = Some(i);
                                    }
                                }
                            },
                        );
                    });
                    ui.separator();
                }

                // Apply mutations
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
                    self.config.actions.swap(idx, idx - 1);
                }
                if let Some(idx) = move_down_idx {
                    self.config.actions.swap(idx, idx + 1);
                }

                ui.add_space(4.0);
                if ui.button("+ Add Action").clicked() {
                    self.action_editor.open_new();
                }
            });

            ui.add_space(12.0);

            // ── Append file path ─────────────────────────────────────
            ui.group(|ui| {
                ui.set_min_width(ui.available_width());
                ui.label(
                    egui::RichText::new("Append File Path").strong().size(16.0),
                );
                ui.add_space(4.0);
                ui.label("URLs will be appended to this file when using 'Append to File' action:");

                let mut path_str = self
                    .config
                    .append_file_path
                    .as_ref()
                    .map(|p| p.display().to_string())
                    .unwrap_or_default();

                if ui
                    .text_edit_singleline(&mut path_str)
                    .changed()
                {
                    if path_str.is_empty() {
                        self.config.append_file_path = None;
                    } else {
                        self.config.append_file_path =
                            Some(std::path::PathBuf::from(&path_str));
                    }
                }
            });

            ui.add_space(12.0);

            // ── System integration ───────────────────────────────────
            ui.group(|ui| {
                ui.set_min_width(ui.available_width());
                ui.label(
                    egui::RichText::new("System Integration")
                        .strong()
                        .size(16.0),
                );
                ui.add_space(4.0);

                ui.horizontal(|ui| {
                    if ui.button("Register as Default Browser").clicked() {
                        match std::env::current_exe() {
                            Ok(exe) => match platform::register_as_default_browser(&exe) {
                                Ok(_) => {
                                    self.status_message =
                                        Some("Registered successfully!".into());
                                    self.status_is_error = false;
                                }
                                Err(e) => {
                                    self.status_message =
                                        Some(format!("Registration failed: {}", e));
                                    self.status_is_error = true;
                                }
                            },
                            Err(e) => {
                                self.status_message =
                                    Some(format!("Cannot determine exe path: {}", e));
                                self.status_is_error = true;
                            }
                        }
                    }

                    if ui.button("Unregister").clicked() {
                        match platform::unregister_as_default_browser() {
                            Ok(_) => {
                                self.status_message =
                                    Some("Unregistered successfully!".into());
                                self.status_is_error = false;
                            }
                            Err(e) => {
                                self.status_message =
                                    Some(format!("Unregistration failed: {}", e));
                                self.status_is_error = true;
                            }
                        }
                    }
                });

                #[cfg(windows)]
                {
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new(
                            "After registering, go to Windows Settings > Apps > \
                             Default apps > Web browser and select URL Dispatcher.",
                        )
                        .weak(),
                    );
                }
            });

            ui.add_space(12.0);

            // ── Save / status ────────────────────────────────────────
            ui.horizontal(|ui| {
                if ui
                    .button(egui::RichText::new("Save Configuration").size(15.0))
                    .clicked()
                {
                    match save_config(&self.config) {
                        Ok(_) => {
                            self.status_message = Some("Configuration saved!".into());
                            self.status_is_error = false;
                        }
                        Err(e) => {
                            self.status_message =
                                Some(format!("Failed to save: {}", e));
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

        // ── Action editor window ─────────────────────────────────────
        if self.action_editor.active {
            self.render_action_editor(ctx);
        }
    }

    fn render_action_editor(&mut self, ctx: &egui::Context) {
        let mut open = self.action_editor.active;
        egui::Window::new(if self.action_editor.editing_id.is_some() {
            "Edit Action"
        } else {
            "Add Action"
        })
        .open(&mut open)
        .resizable(false)
        .collapsible(false)
        .min_width(350.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Type:");
                egui::ComboBox::from_id_salt("action_type_combo")
                    .selected_text(match self.action_editor.action_type {
                        ActionTypeChoice::CopyToClipboard => "Copy to Clipboard",
                        ActionTypeChoice::AppendToFile => "Append to File",
                        ActionTypeChoice::OpenInBrowser => "Open in Browser",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.action_editor.action_type,
                            ActionTypeChoice::CopyToClipboard,
                            "Copy to Clipboard",
                        );
                        ui.selectable_value(
                            &mut self.action_editor.action_type,
                            ActionTypeChoice::AppendToFile,
                            "Append to File",
                        );
                        ui.selectable_value(
                            &mut self.action_editor.action_type,
                            ActionTypeChoice::OpenInBrowser,
                            "Open in Browser",
                        );
                    });
            });

            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut self.action_editor.name);
            });

            if self.action_editor.action_type == ActionTypeChoice::OpenInBrowser {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label("Executable:");
                    ui.text_edit_singleline(&mut self.action_editor.executable_path);
                });

                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label("Arguments:");
                    ui.text_edit_singleline(&mut self.action_editor.args_str);
                });
                ui.label(
                    egui::RichText::new("Use {URL} as placeholder for the URL. Example: --incognito {URL}")
                        .weak()
                        .size(11.0),
                );
            }

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                let name_empty = self.action_editor.name.trim().is_empty();
                let exe_empty = self.action_editor.action_type
                    == ActionTypeChoice::OpenInBrowser
                    && self.action_editor.executable_path.trim().is_empty();

                let can_save = !name_empty && !exe_empty;

                if ui
                    .add_enabled(can_save, egui::Button::new("Save"))
                    .clicked()
                {
                    let new_action = self.action_editor.build_action();
                    if let Some(edit_id) = self.action_editor.editing_id {
                        // Replace existing
                        if let Some(pos) =
                            self.config.actions.iter().position(|a| a.id() == edit_id)
                        {
                            let was_enabled = self.config.actions[pos].enabled();
                            let mut action = new_action;
                            action.set_enabled(was_enabled);
                            self.config.actions[pos] = action;
                        }
                    } else {
                        self.config.actions.push(new_action);
                    }
                    self.action_editor.active = false;
                }

                if ui.button("Cancel").clicked() {
                    self.action_editor.active = false;
                }
            });
        });
        self.action_editor.active = open;
    }
}
