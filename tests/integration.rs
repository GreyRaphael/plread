use std::process::Command;

fn plread_exe() -> Command {
    Command::new(env!("CARGO_BIN_EXE_plread"))
}

#[test]
fn test_help() {
    let output = plread_exe().args(["--help"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("plread"));
    assert!(stdout.contains("ipc"));
    assert!(stdout.contains("parquet"));
}

#[test]
fn test_ipc_subcommand_help() {
    let output = plread_exe().args(["ipc", "--help"]).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("--max-rows"));
}

#[test]
fn test_no_match_returns_error() {
    let output = plread_exe()
        .args(["ipc", "nonexistent_dir/*.ipc"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Error"));
}
