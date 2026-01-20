use std::{
    fs::File,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use google_drive3::{DriveHub, hyper_util::client::legacy::connect::HttpConnector};
use hyper_rustls::HttpsConnector;
use mime_guess::mime;

fn ask_for_file_path() -> PathBuf {
    let mut input = String::new();
    let mut file_path = PathBuf::new();

    let mut is_file_valid = false;

    while !is_file_valid {
        input.clear();

        println!("1️⃣. Please enter the path of the file you want to upload: 🥹");
        std::io::stdin()
            .read_line(&mut input)
            .expect("❌. Failed to read the file path input! 😭");

        let trimmed_input = input.trim().trim_matches('"').trim_matches('\'');
        let temp_path = Path::new(trimmed_input);

        match File::open(temp_path) {
            Ok(_file) => {
                file_path = temp_path.to_path_buf();
                is_file_valid = true;
            }
            Err(err) => match err.kind() {
                ErrorKind::NotFound => {
                    println!("❌. Error: The file '{}' does not exist. 😭", trimmed_input);
                }
                ErrorKind::PermissionDenied => {
                    println!(
                        "❌. Error: You don't have permission to access '{}'. 😭",
                        trimmed_input
                    );
                }
                _ => {
                    println!("❌. Error: An unexpected error occurred: {} 😭", err);
                }
            },
        }
    }

    file_path
}

fn ask_for_file_name(file_path: &PathBuf) -> String {
    let mut input = String::new();
    println!("2️⃣. Change file name or skip by clicking 'Enter': 🥹");
    std::io::stdin()
        .read_line(&mut input)
        .expect("❌. Failed to read file_name input! 😭");

    let trimmed_input = input.trim();

    let uploaded_file_name: &str = if trimmed_input.is_empty() {
        file_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("default_file.bin")
    } else {
        trimmed_input
    };

    uploaded_file_name.to_string()
}

pub async fn upload_file(hub: &DriveHub<HttpsConnector<HttpConnector>>) {
    let file_path = ask_for_file_path();

    let mime_type = match mime_guess::from_path(&file_path).first() {
        Some(mime) => {
            println!("file mime type: {}", mime);
            mime
        }
        None => {
            println!("Error guessing file type, default to TEXT_PLAIN_UTF_8.");
            mime::TEXT_PLAIN_UTF_8
        }
    };

    let file_name = ask_for_file_name(&file_path);

    // get data bytes from selected file
    let data_bytes = File::open(&file_path).expect("❌❌❌ Failed to open file! ❌❌❌");

    // upload file to drive
    let new_file = google_drive3::api::File {
        name: Some(file_name),
        ..Default::default()
    };

    println!("🔄. File uploading, please wait 🫶");
    let result = hub
        .files()
        .create(new_file)
        .upload(data_bytes, mime_type)
        .await
        .expect("❌❌❌ Failed to upload file to Drive! ❌❌❌")
        .1;

    println!(
        "✅. File uploaded successfully~ New file ID: {:?} 🥳",
        result.id
    );
}
