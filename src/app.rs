use eframe::egui;

use crate::config::Config;
use crate::ui_settings::ActionEditor;

#[derive(Debug, Clone)]
pub enum AppMode {
    Dispatch(String),
    Settings,
}

pub struct App {
    pub mode: AppMode,
    pub config: Config,
    pub status_message: Option<String>,
    pub status_is_error: bool,
    pub should_close: bool,
    pub action_editor: ActionEditor,
}

impl App {
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

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.should_close {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }

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
