# plread

CLI tool to preview Arrow IPC, IPC Stream, and Parquet files.

## Install

```bash
cargo install --path .
```

## Usage

```bash
# IPC with glob (LazyFrame scan_ipc, native glob)
plread ipc "data/*.feather"

# IPC Stream (eager, manual glob)
plread ipc-stream "logs/*.arrow"

# Parquet with glob (LazyFrame scan_parquet, native glob)
plread parquet "data/*.parquet"

# Show more rows
plread ipc file.feather --head 10 --tail 10

# Show all columns
plread parquet data.parquet --all-columns
```

## Options

| Flag | Default | Description |
|------|---------|-------------|
| `--head N` | 5 | Number of head rows |
| `--tail N` | 5 | Number of tail rows |
| `--all-columns` | off | Show all columns (disable truncation) |
| `--width N` | 10 | Columns per side when truncated |

## Architecture

- **IPC / Parquet**: Uses Polars LazyFrame `scan_*` API with native glob support + query optimizer
- **IPC Stream**: Uses eager `IpcStreamReader` with manual glob expansion + `concat_df_diagonal`
- All files matched by glob are automatically concatenated
