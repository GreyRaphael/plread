use polars::prelude::*;

/// Read Parquet files using LazyFrame::scan_parquet (native glob support)
pub fn read_parquet(pattern: &str) -> PolarsResult<DataFrame> {
    let lf = LazyFrame::scan_parquet(
        pattern.into(),
        ScanArgsParquet {
            glob: true,
            ..Default::default()
        },
    )?;
    lf.collect()
}
