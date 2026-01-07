use anyhow::Result;
use polars::prelude::*;
use std::fs::File;
use std::path::PathBuf;

pub fn pick_file() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .add_filter("Parquet", &["parquet"])
        .pick_file()
}

pub fn read_parquet(path: &PathBuf) -> Result<DataFrame> {
    let file = File::open(path)?;
    let df = ParquetReader::new(file).finish()?;
    Ok(df)
}
