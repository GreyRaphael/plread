use polars::prelude::*;
use terminal_size::terminal_size;

/// Configure polars env vars before printing a DataFrame.
/// Returns (terminal_width, max_cols) for display purposes.
fn setup_polars_display(max_rows: usize) -> (usize, usize) {
    // SAFETY: single-threaded CLI startup, no concurrent env access
    unsafe {
        std::env::set_var("POLARS_FMT_TABLE_HIDE_DATAFRAME_SHAPE_INFORMATION", "1");
        std::env::set_var("POLARS_FMT_MAX_ROWS", max_rows.to_string());

        let width = terminal_size().map(|(w, _)| w.0 as usize).unwrap_or(120);
        std::env::set_var("POLARS_TABLE_WIDTH", width.to_string());
        let max_cols = width / 15;
        std::env::set_var("POLARS_FMT_MAX_COLS", max_cols.to_string());
        (width, max_cols)
    }
}

pub fn display_df(df: &DataFrame, max_rows: usize) {
    let (term_width, max_cols) = setup_polars_display(max_rows);

    println!(
        "Shape: {} rows x {} columns (terminal: {}w, dynamic display limit: {} cols)",
        df.height(),
        df.width(),
        term_width,
        max_cols
    );
    println!("{}", df);
}
