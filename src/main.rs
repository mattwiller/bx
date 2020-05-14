#![warn(rust_2018_idioms)]
#![deny(clippy::all)]

mod sdk;

use clap::{App, AppSettings, Arg, ArgGroup, SubCommand};
use sdk::operations::FileUpdates;
use sdk::Client;
use sdk::SDKError;
use serde::Serialize;
use serde_json;
use std::fmt::Debug;
use std::path::Path;

enum OutputFormat {
    Debug,
    JSON,
}

struct Context {
    client: Client,
    fmt: OutputFormat,
}

impl Context {
    pub(crate) fn output<T>(&self, object: T)
    where
        T: Serialize + Debug,
    {
        match self.fmt {
            OutputFormat::Debug => println!("{:?}", object),
            OutputFormat::JSON => println!("{}", &serde_json::to_string(&object).unwrap()),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        .group(ArgGroup::with_name("output_format"))
        .arg(
            Arg::with_name("json")
                .long("json")
                .help("Format output as JSON")
                .takes_value(false)
                .global(true)
                .group("output_format"),
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

    let mut context = Context {
        client: Client::new(token.to_owned()),
        fmt: if matches.is_present("json") {
            OutputFormat::JSON
        } else {
            OutputFormat::Debug
        },
    };

    // OBJECT: file
    if let Some(matches) = matches.subcommand_matches("file") {
        let file_id = matches.value_of("fileID").unwrap();

        // ACTION: delete
        if matches.is_present("delete") {
            delete_file(&mut context, file_id).await?;
        // ACTION: download
        } else if matches.is_present("downloadToPath") {
            let path = Path::new(matches.value_of("downloadToPath").unwrap_or("."));
            download_file(&mut context, file_id, path).await?;
        // ACTION: update
        } else if matches.is_present("update") {
            let mut updates = FileUpdates::new();
            if let Some(name) = matches.value_of("name") {
                updates = updates.name(name);
            }
            if let Some(description) = matches.value_of("description") {
                updates = updates.description(description);
            }

            update_file(&mut context, file_id, updates).await?;
        // DEFAULT ACTION: get
        } else {
            get_file(&mut context, file_id).await?;
        }

    // COMMAND: upload
    } else if let Some(matches) = matches.subcommand_matches("upload") {
        let path = Path::new(matches.value_of("path").unwrap());
        let folder_id = matches.value_of("folderID").unwrap_or("0");
        upload_file(&mut context, path, folder_id).await?;

    // COMMAND: user
    } else if let Some(matches) = matches.subcommand_matches("user") {
        let id = matches.value_of("id").unwrap();
        get_user(&mut context, id).await?;
    }

    Ok(())
}

async fn get_file(ctx: &mut Context, id: &str) -> Result<(), SDKError> {
    println!("Fetching file {}", id);
    let file = ctx.client.file(id).get().await?;
    ctx.output(file);
    Ok(())
}

async fn update_file(ctx: &mut Context, id: &str, updates: FileUpdates) -> Result<(), SDKError> {
    println!("Updating file {}", id);
    let file = ctx.client.file(id).update(updates).await?;
    ctx.output(file);
    Ok(())
}

async fn download_file(ctx: &mut Context, id: &str, path: &Path) -> Result<(), SDKError> {
    println!("Downloading file {}", id);
    ctx.client.file(id).download(path).await?;
    println!("File {} downloaded to {}", id, path.to_str().unwrap());
    Ok(())
}

async fn delete_file(ctx: &mut Context, id: &str) -> Result<(), SDKError> {
    println!("Deleting file {}", id);
    ctx.client.file(id).delete().await?;
    println!("File {} deleted", id);
    Ok(())
}

async fn upload_file(ctx: &mut Context, path: &Path, folder_id: &str) -> Result<(), SDKError> {
    let file = ctx.client.upload_file(path, folder_id).await?;
    ctx.output(file);
    Ok(())
}

async fn get_user(ctx: &mut Context, id: &str) -> Result<(), SDKError> {
    let user = ctx.client.user(id).get().await?;
    ctx.output(user);
    Ok(())
}
