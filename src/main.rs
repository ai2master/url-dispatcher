// 入口：解析命令行参数，决定分发模式或设置模式 | Entry: parse CLI args, choose dispatch or settings mode

mod actions;
mod app;
mod config;
mod i18n;
mod platform;
mod ui_dispatch;
mod ui_settings;

use app::{App, AppMode};

fn main() -> eframe::Result {
    let args: Vec<String> = std::env::args().collect();

    let mode = if args.len() > 1 {
        AppMode::Dispatch(args[1].clone())
    } else {
        AppMode::Settings
    };

    let cfg = config::load_config().unwrap_or_default();

    let (title, width, height, is_dispatch) = match &mode {
        AppMode::Dispatch(url) => {
            let display_url = if url.len() > 60 {
                format!("{}...", &url[..57])
            } else {
                url.clone()
            };
            (
                format!("URL Dispatcher - {}", display_url),
                420.0,
                350.0,
                true,
            )
        }
        AppMode::Settings => ("URL Dispatcher - Settings".to_string(), 650.0, 550.0, false),
    };

    let mut viewport = eframe::egui::ViewportBuilder::default()
        .with_inner_size([width, height])
        .with_resizable(!is_dispatch);
    if is_dispatch {
        viewport = viewport.with_always_on_top();
    }

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        &title,
        options,
        Box::new(move |_cc| Ok(Box::new(App::new(mode, cfg)))),
    )
}
