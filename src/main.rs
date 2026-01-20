use colored::Colorize;

use crate::services::SERVICES;

mod file_upload;
mod google_drive;
mod services;

#[tokio::main]
async fn main() {
    let service = services::ask_user_for_service();
    let hub = google_drive::init_google_drive().await;

    match service {
        SERVICES::DisplayFiles => println!(
            "{}",
            "Currently this service is not available right now! 🤪".green()
        ),
        SERVICES::UploadFile => file_upload::upload_file(&hub).await,
    }
}
