mod file_upload;
mod google_drive;

#[tokio::main]
async fn main() {
    let hub = google_drive::init_google_drive().await;
    file_upload::upload_file(&hub).await;
}
