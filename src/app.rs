use crate::fonts;
use crate::parquet_io;
use crate::ui as app_ui;
use eframe::egui;
use polars::prelude::*;

pub struct ParquetViewerApp {
    dataframe: Option<DataFrame>,
    error_message: Option<String>,
}

impl ParquetViewerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        fonts::configure_fonts(&cc.egui_ctx);
        Self {
            dataframe: None,
            error_message: None,
        }
    }

    fn open_file(&mut self) {
        if let Some(path) = parquet_io::pick_file() {
            match parquet_io::read_parquet(&path) {
                Ok(df) => {
                    self.dataframe = Some(df);
                    self.error_message = None;
                }
                Err(e) => {
                    self.error_message = Some(format!("Failed to read parquet: {}", e));
                }
            }
        }
    }
}

impl eframe::App for ParquetViewerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open Parquet File").clicked() {
                        self.open_file();
                        ui.close_menu();
                    }
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(err) = &self.error_message {
                ui.label(egui::RichText::new(format!("Error: {}", err)).color(egui::Color32::RED));
                ui.separator();
            }

            if let Some(df) = &self.dataframe {
                ui.label(format!("Loaded DataFrame with shape: {:?}", df.shape()));
                app_ui::render_table(ui, df);
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label("No Parquet file loaded. Use File > Open to load one.");
                });
            }
        });
    }
}
