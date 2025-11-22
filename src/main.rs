#![windows_subsystem = "windows"]

mod app;
mod system_info;

fn main() -> eframe::Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions {
        centered: true,
        viewport: egui::ViewportBuilder {
            //inner_size: Some(egui::vec2(app::WIN_WIDTH, app::WIN_HEIGHT)),
            decorations: Some(false),
            minimize_button: Some(false),
            maximize_button: Some(false),
            transparent: Some(true),
            window_level: Some(egui::WindowLevel::AlwaysOnTop),

            ..Default::default()
        },
        ..Default::default()
    };

    eframe::run_native(
        "RAM Status Monitor",
        native_options,
        Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
    )
}
