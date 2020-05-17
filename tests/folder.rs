use httpmock::Method::GET;
use httpmock::{mock, with_mock_server};
use serde_json::json;

mod common;
use common::{assert_output_json, cmd};

#[test]
#[with_mock_server]
fn bx_folder_displays_root_folder() {
    let mut cmd = cmd();
    cmd.args(&["folder", "-t", "access_token", "--json"]);

    let json = json!({
        "type": "folder",
        "id": "0",
        "etag": null,
        "name": "All Files",
        "sequence_id": null
    });

    let search_mock = mock(GET, "/folders/0")
        .expect_header("authorization", "Bearer access_token")
        .return_status(200)
        .return_json_body(&json)
        .create();

    let expected = json!({
        "id": "0",
        "etag": null,
        "name": "All Files",
        "sequence_id": null
    });
    assert_output_json(cmd, expected);
    assert_eq!(search_mock.times_called(), 1);
}

#[test]
#[with_mock_server]
fn bx_folder_id_displays_folder() {
    let mut cmd = cmd();
    cmd.args(&["folder", "3", "-t", "access_token", "--json"]);

    let json = json!({
        "type": "folder",
        "id": "3",
        "etag": "1",
        "name": "Test Folder",
        "sequence_id": "1"
    });

    let search_mock = mock(GET, "/folders/3")
        .expect_header("authorization", "Bearer access_token")
        .return_status(200)
        .return_json_body(&json)
        .create();

    let expected = json!({
        "id": "3",
        "etag": "1",
        "name": "Test Folder",
        "sequence_id": "1"
    });
    assert_output_json(cmd, expected);
    assert_eq!(search_mock.times_called(), 1);
}
