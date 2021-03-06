use httpmock::Method::GET;
use httpmock::{mock, with_mock_server};
use serde_json::json;

mod common;
use common::{assert_output_json, cmd};

#[test]
#[with_mock_server]
fn bx_user_displays_current_user() {
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
fn bx_user_id_displays_user() {
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
