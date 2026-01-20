use colored::Colorize;

use crate::services::SERVICES;

mod explore_files;
mod file_upload;
mod google_drive;
mod services;

#[tokio::main]
async fn main() {
    let service = services::ask_user_for_service();
    let hub = google_drive::init_google_drive().await;

    match service {
        SERVICES::DisplayFiles => explore_files::explore(&hub).await,
        SERVICES::UploadFile => file_upload::upload_file(&hub).await,
    }
}
