#![warn(rust_2018_idioms)]
#![deny(clippy::all)]

mod sdk;

use tokio::prelude::*;

use clap::{App, AppSettings, Arg, ArgGroup, SubCommand};
use reqwest::Response;
use sdk::file::FileUpdates;
use sdk::Client;
use serde_json::json;
use std::error::Error;
use std::path::Path;
use tokio::fs;
use tokio_util::codec::{BytesCodec, FramedRead};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("token")
                .long("token")
                .short("t")
                .value_name("TOKEN")
                .help("Access token to use for any API requests")
                .takes_value(true)
                .global(true),
        )
        .subcommand(
            SubCommand::with_name("file")
                .about("Displays information about a file")
                .arg(
                    Arg::with_name("fileID")
                        .help("The ID of the file")
                        .required(true),
                )
                .group(ArgGroup::with_name("action").args(&["delete", "downloadToPath"]))
                .arg(
                    Arg::with_name("delete")
                        .help("Delete the file")
                        .short("D")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("downloadToPath")
                        .help("Download a file to the provided path")
                        .long("download-to")
                        .takes_value(true),
                )
                .group(
                    ArgGroup::with_name("update")
                        .args(&["name", "description"])
                        .multiple(true)
                        .conflicts_with("action"),
                )
                .arg(
                    Arg::with_name("name")
                        .help("Sets the name of the file")
                        .long("name")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("description")
                        .help("Sets the description of the file")
                        .long("description")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("upload")
                .about("Upload a file")
                .arg(
                    Arg::with_name("path")
                        .help("The path of the file on disk")
                        .required(true),
                )
                .arg(
                    Arg::with_name("folderID")
                        .long("folder-id")
                        .short("D")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("user")
                .about("Display information about a user")
                .arg(
                    Arg::with_name("id")
                        .help("The ID of the user")
                        .default_value("me"),
                ),
        );

    let matches = app.get_matches();

    let token = matches.value_of("token").expect("Token must be provided!");
    let mut client = Client::new(token.to_owned());

    // OBJECT: file
    if let Some(matches) = matches.subcommand_matches("file") {
        let file_id = matches.value_of("fileID").unwrap();

        // ACTION: delete
        if matches.is_present("delete") {
            delete_file(&mut client, file_id).await?;
        // ACTION: download
        } else if matches.is_present("downloadToPath") {
            let path = Path::new(matches.value_of("downloadToPath").unwrap_or("."));
            download_file(&mut client, file_id, path).await?;
        // ACTION: update
        } else if matches.is_present("update") {
            let mut updates = FileUpdates::new();
            if let Some(name) = matches.value_of("name") {
                updates = updates.name(name);
            }
            if let Some(description) = matches.value_of("description") {
                updates = updates.description(description);
            }

            update_file(&mut client, file_id, updates).await?;
        // DEFAULT ACTION: get
        } else {
            get_file(&mut client, file_id).await?;
        }

    // COMMAND: upload
    } else if let Some(matches) = matches.subcommand_matches("upload") {
        let path = Path::new(matches.value_of("path").unwrap());
        let folder_id = matches.value_of("folderID").unwrap_or("0");
        upload_file(&mut client, path, folder_id).await?;

        // COMMAND: user
    } else if let Some(matches) = matches.subcommand_matches("user") {
        let id = matches.value_of("id").unwrap();
        get_user(&mut client, id).await?;
    }

    Ok(())
}

async fn get_file(client: &mut Client, id: &str) -> Result<(), sdk::Error> {
    println!("Fetching file {}", id);
    let file = client.get_file(id).await?;
    println!("{:?}", file);
    Ok(())
}

async fn update_file(
    client: &mut Client,
    id: &str,
    updates: FileUpdates<'_>,
) -> Result<(), Box<dyn Error>> {
    println!("Updating file {}", id);
    let url = format!("https://api.box.com/2.0/files/{}", id);

    let resp = client.put(&url, updates).await?;
    println!("{:#?}", resp.text().await?);
    Ok(())
}

async fn download_file(client: &mut Client, id: &str, path: &Path) -> Result<(), Box<dyn Error>> {
    println!("Downloading file {}", id);
    let url = format!("https://api.box.com/2.0/files/{}/content", id);

    let mut resp: Response = client.get(&url).await?;

    let mut file = tokio::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)
        .await?;

    while let Some(bytes) = resp.chunk().await? {
        file.write_all(&bytes).await?;
    }
    println!("File {} downloaded to {}", id, path.to_str().unwrap());
    Ok(())
}

async fn delete_file(client: &mut Client, id: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Deleting file {}", id);
    let url = format!("https://api.box.com/2.0/files/{}", id);

    client.delete(&url).await?;
    println!("File {} deleted", id);
    Ok(())
}

async fn upload_file(
    client: &mut Client,
    path: &Path,
    folder_id: &str,
) -> Result<(), Box<dyn Error>> {
    let file = fs::File::open(path).await?;
    let stream = FramedRead::new(file, BytesCodec::new());
    let file_part = reqwest::multipart::Part::stream(reqwest::Body::wrap_stream(stream))
        .file_name("UNUSED")
        .mime_str("application/octet-stream")
        .unwrap();

    let filename = path.file_name().unwrap().to_str();
    let attributes_json = json!({
        "name": filename,
        "parent": {
            "id": folder_id
        }
    })
    .to_string();

    let form = reqwest::multipart::Form::new()
        .text("attributes", attributes_json)
        .part("file", file_part);

    let url = "https://upload.box.com/api/2.0/files/content";

    let resp = client.multipart_upload(&url, form).await?;
    println!("{:#?}", resp.text().await?);
    Ok(())
}

async fn get_user(client: &mut Client, id: &str) -> Result<(), Box<dyn Error>> {
    let user = client.get_user(id).await?;
    println!("{:?}", user);
    Ok(())
}
