use polars::prelude::*;

pub struct DisplayOptions {
    pub head: usize,
    pub tail: usize,
    pub all_columns: bool,
    pub width: usize,
}

/// Disable polars' own column/row truncation so our logic takes full control.
fn disable_polars_truncation() {
    // Prevent polars Display from truncating columns based on terminal width
    // SAFETY: single-threaded CLI startup, no concurrent env access
    unsafe {
        std::env::set_var("POLARS_FMT_MAX_COLS", "-1");
        std::env::set_var("POLARS_FMT_TABLE_HIDE_DATAFRAME_SHAPE_INFORMATION", "1");
    }
}

pub fn display_df(df: &DataFrame, opts: &DisplayOptions) {
    disable_polars_truncation();

    let total_rows = df.height();
    let total_cols = df.width();
    let col_names = df.get_column_names();

    // Header
    println!("Shape: {} rows x {} columns", total_rows, total_cols);

    // Column truncation (our logic, not polars')
    let (show_df, col_msg) = if !opts.all_columns && total_cols > opts.width * 2 {
        let left: Vec<&str> = col_names[..opts.width].iter().map(|s| s.as_ref()).collect();
        let right: Vec<&str> = col_names[total_cols - opts.width..]
            .iter()
            .map(|s| s.as_ref())
            .collect();
        let mut selected = left.clone();
        selected.extend(&right);
        let msg = format!(
            "Showing {}/{} columns: {} ... {}",
            opts.width * 2,
            total_cols,
            left.join(", "),
            right.join(", ")
        );
        (df.select(&selected).unwrap(), Some(msg))
    } else {
        (df.clone(), None)
    };

    if let Some(msg) = col_msg {
        println!("{}", msg);
    }
    // Head
    println!("{}", show_df);
}
