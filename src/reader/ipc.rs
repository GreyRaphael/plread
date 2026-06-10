use polars::prelude::*;

/// Read IPC files using LazyFrame::scan_ipc (native glob support)
pub fn read_ipc(pattern: &str) -> PolarsResult<DataFrame> {
    let lf = LazyFrame::scan_ipc(
        pattern.into(),
        Default::default(),
        Default::default(),
    )?;
    lf.collect()
}
