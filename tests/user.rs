use assert_cmd::prelude::*;
use assert_json_diff::assert_json_eq;
use httpmock::Method::GET;
use httpmock::{mock, with_mock_server};
use serde_json::json;
use std::process::Command;

fn cmd() -> Command {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.env("BOX_API_ROOT", "http://localhost:5000");
    cmd
}

fn assert_output_json(mut cmd: Command, expected: serde_json::Value) {
    let result = cmd.output().unwrap();
    assert!(result.status.success());
    let output = serde_json::from_slice::<serde_json::Value>(&result.stdout).unwrap();

    assert_json_eq!(output, expected);
}

#[test]
#[with_mock_server]
fn it_displays_current_user_when_no_id_passed() {
    let mut cmd = cmd();
    cmd.args(&["user", "-t", "access_token", "--json"]);

    let json = json!({
        "type": "user",
        "id": "1",
        "name": "Test User",
        "login": "user@example.com",
    });

    let search_mock = mock(GET, "/users/me")
        .expect_header("authorization", "Bearer access_token")
        .return_status(200)
        .return_json_body(&json)
        .create();

    let expected = json!({
        "id": "1",
        "name": "Test User",
        "login": "user@example.com",
    });
    assert_output_json(cmd, expected);
    assert_eq!(search_mock.times_called(), 1);
}

#[test]
#[with_mock_server]
fn it_displays_user_when_id_passed() {
    let mut cmd = cmd();
    cmd.args(&["user", "2", "-t", "access_token", "--json"]);

    let json = json!({
        "type": "user",
        "id": "2",
        "name": "Test User 2",
        "login": "user2@example.com",
    });

    let search_mock = mock(GET, "/users/2")
        .expect_header("authorization", "Bearer access_token")
        .return_status(200)
        .return_json_body(&json)
        .create();

    let expected = json!({
        "id": "2",
        "name": "Test User 2",
        "login": "user2@example.com",
    });
    assert_output_json(cmd, expected);
    assert_eq!(search_mock.times_called(), 1);
}
