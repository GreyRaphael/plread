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

    /// Maximum number of rows to display (head + tail combined)
    #[arg(long, default_value_t = 10)]
    pub max_rows: usize,
}
