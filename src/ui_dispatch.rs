use eframe::egui;

use crate::actions;
use crate::app::App;
use crate::config::Action;

impl App {
    pub fn render_dispatcher_ui(&mut self, ctx: &egui::Context) {
        let url = match &self.mode {
            crate::app::AppMode::Dispatch(u) => u.clone(),
            _ => return,
        };

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.add_space(8.0);

                // URL display
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("URL:").strong());
                });
                ui.add_space(4.0);

                egui::Frame::group(ui.style())
                    .inner_margin(8.0)
                    .show(ui, |ui| {
                        ui.set_max_width(ui.available_width());
                        let url_text = if url.len() > 120 {
                            format!("{}...", &url[..117])
                        } else {
                            url.clone()
                        };
                        ui.label(
                            egui::RichText::new(&url_text)
                                .monospace()
                                .size(13.0),
                        );
                    });

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(8.0);

                // Action buttons
                let enabled_actions: Vec<(usize, Action)> = self
                    .config
                    .actions
                    .iter()
                    .enumerate()
                    .filter(|(_, a)| a.enabled())
                    .map(|(i, a)| (i, a.clone()))
                    .collect();

                let mut action_to_execute: Option<Action> = None;

                for (display_idx, (_config_idx, action)) in enabled_actions.iter().enumerate() {
                    let shortcut_num = display_idx + 1;
                    let label = format!("[{}] {}", shortcut_num, action.name());

                    let button = egui::Button::new(
                        egui::RichText::new(&label).size(15.0),
                    )
                    .min_size(egui::vec2(ui.available_width(), 32.0));

                    if ui.add(button).clicked() {
                        action_to_execute = Some(action.clone());
                    }

                    // Keyboard shortcut
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

                // Escape to close
                if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                    self.should_close = true;
                }

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(4.0);

                // Bottom row: Settings + Cancel
                ui.horizontal(|ui| {
                    if ui.button("Settings").clicked() {
                        self.mode = crate::app::AppMode::Settings;
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Cancel").clicked() {
                            self.should_close = true;
                        }
                    });
                });

                // Status message
                if let Some(msg) = &self.status_message {
                    ui.add_space(8.0);
                    let color = if self.status_is_error {
                        egui::Color32::RED
                    } else {
                        egui::Color32::GREEN
                    };
                    ui.label(egui::RichText::new(msg).color(color));
                }

                // Execute action if requested
                if let Some(action) = action_to_execute {
                    self.execute_action(&action, &url);
                }
            });
        });
    }

    fn execute_action(&mut self, action: &Action, url: &str) {
        let result = match action {
            Action::CopyToClipboard { .. } => actions::copy_to_clipboard(url),
            Action::AppendToFile { .. } => {
                if let Some(path) = &self.config.append_file_path {
                    actions::append_to_file(url, path)
                } else {
                    Err(anyhow::anyhow!(
                        "Append file path not configured. Please set it in Settings."
                    ))
                }
            }
            Action::OpenInBrowser {
                executable_path,
                args,
                ..
            } => actions::open_in_browser(url, executable_path, args),
        };

        match result {
            Ok(_) => {
                self.should_close = true;
            }
            Err(e) => {
                self.status_message = Some(format!("Error: {}", e));
                self.status_is_error = true;
            }
        }
    }
}
