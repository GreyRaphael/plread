use polars::prelude::*;
use terminal_size::terminal_size;

pub struct DisplayOptions {
    pub max_rows: usize,
}

/// Configure polars env vars before printing a DataFrame.
/// Returns (terminal_width, max_cols) for display purposes.
fn setup_polars_display(max_rows: usize) -> (usize, usize) {
    // SAFETY: single-threaded CLI startup, no concurrent env access
    unsafe {
        std::env::set_var("POLARS_FMT_TABLE_HIDE_DATAFRAME_SHAPE_INFORMATION", "1");
        std::env::set_var("POLARS_FMT_MAX_ROWS", max_rows.to_string());

        let width = terminal_width().unwrap_or(120);
        std::env::set_var("POLARS_TABLE_WIDTH", width.to_string());
        // Each column ~15 chars wide; cap by physical terminal width
        let max_cols = width / 15;
        std::env::set_var("POLARS_FMT_MAX_COLS", max_cols.to_string());
        (width, max_cols)
    }
}

fn terminal_width() -> Option<usize> {
    terminal_size().map(|(w, _)| w.0 as usize)
}

pub fn display_df(df: &DataFrame, opts: &DisplayOptions) {
    let (term_width, max_cols) = setup_polars_display(opts.max_rows);

    let total_rows = df.height();
    let total_cols = df.width();

    println!(
        "Shape: {} rows x {} columns (terminal: {}w, display limit: {} cols)",
        total_rows, total_cols, term_width, max_cols
    );
    println!("{}", df);
}
