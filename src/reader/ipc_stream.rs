use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use polars::prelude::*;

/// Expand glob pattern into file paths
fn expand_glob(pattern: &str) -> Result<Vec<PathBuf>, String> {
    let paths: Vec<PathBuf> = glob::glob(pattern)
        .map_err(|e| format!("Invalid glob '{}': {}", pattern, e))?
        .filter_map(|r| r.ok())
        .collect();

    if paths.is_empty() {
        return Err(format!("No files matched: {}", pattern));
    }
    Ok(paths)
}

/// Read IPC Stream files (eager, manual glob)
pub fn read_ipc_stream(pattern: &str) -> PolarsResult<DataFrame> {
    let paths =
        expand_glob(pattern).map_err(|e| PolarsError::ComputeError(e.into()))?;

    let dfs: Vec<DataFrame> = paths
        .iter()
        .map(|p| {
            let file = File::open(p).map_err(|e| {
                PolarsError::ComputeError(
                    format!("Cannot open {:?}: {}", p, e).into(),
                )
            })?;
            IpcStreamReader::new(BufReader::new(file)).finish()
        })
        .collect::<PolarsResult<Vec<_>>>()?;

    if dfs.len() == 1 {
        Ok(dfs.into_iter().next().unwrap())
    } else {
        polars::functions::concat_df_diagonal(&dfs)
    }
}
