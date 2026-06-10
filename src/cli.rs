use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "plread", version, about = "Preview Arrow IPC / IPC Stream / Parquet files")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Format,
}

#[derive(Subcommand, Debug)]
pub enum Format {
    /// Read Arrow IPC files (LazyFrame scan, native glob)
    Ipc(CommonArgs),
    /// Read Arrow IPC Stream files (eager, manual glob)
    IpcStream(CommonArgs),
    /// Read Parquet files (LazyFrame scan, native glob)
    Parquet(CommonArgs),
}

#[derive(Parser, Debug)]
pub struct CommonArgs {
    /// File path pattern (supports glob, e.g. "path/*.ipc")
    pub pattern: String,

    /// Number of head rows to display
    #[arg(long, default_value_t = 5)]
    pub head: usize,

    /// Number of tail rows to display
    #[arg(long, default_value_t = 5)]
    pub tail: usize,

    /// Show all columns (disable truncation)
    #[arg(long)]
    pub all_columns: bool,

    /// Number of columns to show on each side when truncated
    #[arg(long, default_value_t = 10)]
    pub width: usize,
}
