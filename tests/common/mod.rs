use assert_cmd::Command;
use assert_json_diff::assert_json_eq;

pub fn assert_output_json(mut cmd: Command, expected: serde_json::Value) {
    let result = cmd.output().unwrap();
    println!("{:?}", result);
    let output = serde_json::from_slice::<serde_json::Value>(&result.stdout).unwrap();

    assert_json_eq!(output, expected);
    assert!(result.status.success());
}

pub fn cmd() -> Command {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.env("BOX_API_ROOT", "http://localhost:5000");
    cmd
}
