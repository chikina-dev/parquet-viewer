mod app;
mod fonts;
mod parquet_io;
mod ui;

use app::ParquetViewerApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Parquet Viewer",
        native_options,
        Box::new(|cc| Ok(Box::new(ParquetViewerApp::new(cc)))),
    )
}
