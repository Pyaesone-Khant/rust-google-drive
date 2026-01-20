use std::process;

use colored::Colorize;
use google_drive3::{DriveHub, hyper_util::client::legacy::connect::HttpConnector};
use hyper_rustls::HttpsConnector;

#[derive(Debug)]
pub enum FILE_TYPE {
    Image,
    Video,
    Pdf,
}

fn ask_file_type() -> FILE_TYPE {
    let mut input = String::new();

    let mut file_type: Option<FILE_TYPE> = None;

    while file_type.is_none() {
        input.clear();
        println!("{}", "Select your desired file type: 😉".green().bold());
        println!(
            "
|-----------------------------------|
|                                   |
|       1️⃣. {}                    |
|       2️⃣. {}                    |
|       3️⃣. {}                      |
|       4️⃣. {}             |
|                                   |
|-----------------------------------|        
    ",
            "Image".yellow().bold(),
            "Video".green().bold(),
            "Pdf".blue().bold(),
            "Quit Program".red().bold()
        );

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read file type input!");

        match input.trim().parse::<i32>() {
            Ok(num) => match num {
                1 => file_type = Some(FILE_TYPE::Image),
                2 => file_type = Some(FILE_TYPE::Video),
                3 => file_type = Some(FILE_TYPE::Pdf),
                4 => process::exit(0),
                _ => println!("❌. Service not available! Please select the available service. 😭"),
            },
            Err(_err) => {
                println!("❌. Invalid input received, please select the service again. 😭")
            }
        }
    }

    file_type.unwrap()
}

pub async fn explore(hub: &DriveHub<HttpsConnector<HttpConnector>>) {
    let file_type = ask_file_type();

    let mime_query = match file_type {
        FILE_TYPE::Image => "mimeType contains 'image/'",
        FILE_TYPE::Video => "mimeType contains 'video/'",
        FILE_TYPE::Pdf => "mimeType = 'application/pdf'",
        // _ => "trashed = false",
    };

    let final_query = format!("{} and trashed = false", mime_query);

    println!(
        "{}",
        format!("🔄. Fetching the latest 5 {:?}, please wait 🥹", file_type)
            .green()
            .bold()
    );
    let result = hub
        .files()
        .list()
        .q(&final_query)
        .page_size(5)
        .param("fields", "files(id, name, mimeType, webContentLink)")
        .add_scope("https://www.googleapis.com/auth/drive.readonly")
        .doit()
        .await;

    match result {
        Ok((_response, file_list)) => {
            if let Some(files) = file_list.files {
                println!("Found {} files:", files.len());
                for (index, file) in files.iter().enumerate() {
                    println!(
                        "{}",
                        format!(
                            "{} - url:{:?} - name:{} - id:({})",
                            index + 1,
                            file.web_content_link,
                            file.name.as_deref().unwrap_or_default(),
                            file.id.as_deref().unwrap_or_default()
                        )
                        .cyan()
                    );
                }
            } else {
                println!("No files found.");
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
