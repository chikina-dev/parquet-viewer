use eframe::egui;
use egui_extras::{Column, TableBuilder};
use geozero::ToWkt;
use polars::prelude::*;

pub fn render_table(ui: &mut egui::Ui, df: &DataFrame) {
    let text_height = egui::TextStyle::Body.resolve(ui.style()).size;
    let num_rows = df.height();
    let columns = df.get_column_names();

    let available_width = ui.available_width().max(300.0) - 50.0;
    let col_count = columns.len().max(1) as f32;
    let col_width = (available_width / col_count).max(50.0);

    TableBuilder::new(ui)
        .striped(true)
        .resizable(true)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Column::auto())
        .columns(
            Column::initial(col_width).resizable(true).clip(true),
            columns.len(),
        )
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.strong("Row");
            });
            for col_name in &columns {
                header.col(|ui| {
                    ui.strong(col_name.as_str());
                });
            }
        })
        .body(|body| {
            body.rows(text_height, num_rows, |mut row| {
                let row_index = row.index();
                row.col(|ui| {
                    ui.label(row_index.to_string());
                });

                for (col_idx, col_name) in columns.iter().enumerate() {
                    row.col(|ui| {
                        if let Ok(col) = df.column(col_name.as_str()) {
                            let val = col.get(row_index);
                            if let Ok(val) = val {
                                let text = format_value(col_name, &val);

                                egui::ScrollArea::horizontal()
                                    .id_salt(format!("scroll_{}_{}", row_index, col_idx))
                                    .show(ui, |ui| {
                                        ui.add(egui::Label::new(&text));
                                    });
                            } else {
                                ui.label("Error");
                            }
                        }
                    });
                }
            });
        });
}

fn format_value(col_name: &str, val: &AnyValue) -> String {
    if col_name == "geometry" {
        if let AnyValue::Binary(bytes) = val {
            if let Ok(wkt) = geozero::wkb::Wkb(bytes.to_vec()).to_wkt() {
                return wkt;
            }
            if let Ok(wkt) = geozero::wkb::Wkb(bytes).to_wkt() {
                return wkt;
            }
        }
    }
    format!("{}", val)
}
