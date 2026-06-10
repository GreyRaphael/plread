mod cli;
mod display;
mod reader;

use clap::Parser;
use cli::{Cli, Format};
use display::{DisplayOptions, display_df};

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let (df, args) = match &cli.command {
        Format::Ipc(args) => (reader::ipc::read_ipc(&args.pattern)?, args),
        Format::IpcStream(args) => {
            (reader::ipc_stream::read_ipc_stream(&args.pattern)?, args)
        }
        Format::Parquet(args) => {
            (reader::parquet::read_parquet(&args.pattern)?, args)
        }
    };

    let opts = DisplayOptions {
        max_rows: args.max_rows,
    };

    if df.height() == 0 {
        println!("Warning: DataFrame is empty (0 rows)");
    } else {
        display_df(&df, &opts);
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
