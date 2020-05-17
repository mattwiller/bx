use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn it_outputs_help_when_run_without_args() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    cmd.assert().failure();
    cmd.assert().stderr(predicates::str::starts_with(format!(
        "bx {}\n\
        Matt Willer <matt.r.willer@gmail.com>\n\
        A smaller, faster Box CLI\n\
    ",
        env!("CARGO_PKG_VERSION")
    )));
}
