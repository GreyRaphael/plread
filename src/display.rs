use polars::prelude::*;

pub struct DisplayOptions {
    pub head: usize,
    pub tail: usize,
    pub all_columns: bool,
    pub width: usize,
}

pub fn display_df(df: &DataFrame, opts: &DisplayOptions) {
    let total_rows = df.height();
    let total_cols = df.width();
    let col_names = df.get_column_names();

    // Header
    println!("Shape: {} rows x {} columns", total_rows, total_cols);

    // Column truncation
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
    println!();

    // Row truncation
    let head_n = opts.head.min(total_rows);
    let tail_n = opts.tail.min(total_rows);
    let needs_ellipsis = total_rows > head_n + tail_n;

    // Head
    println!("{}", show_df.head(Some(head_n)));

    if needs_ellipsis {
        let hidden = total_rows - head_n - tail_n;
        println!("... ({} rows omitted) ...\n", hidden);
        println!("{}", show_df.tail(Some(tail_n)));
    }
}
