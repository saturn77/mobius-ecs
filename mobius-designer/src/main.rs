use eframe::{egui, NativeOptions};
use mobius_designer::DesignerApp;

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("Mobius Designer"),
        ..Default::default()
    };

    let mut app = DesignerApp::new();

    eframe::run_native(
        "Mobius Designer",
        options,
        Box::new(|_cc| Ok(Box::new(app))),
    )
}