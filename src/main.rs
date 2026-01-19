extern crate hyper;
extern crate hyper_rustls;
use google_drive3::{DriveHub, hyper_util, yup_oauth2};
use rustls::crypto::aws_lc_rs;
use std::path::Path;

#[tokio::main]
async fn main() {
    // creating crypto provider
    aws_lc_rs::default_provider()
        .install_default()
        .expect("Failed to install default provider");

    // get application secret
    let secret_path = Path::new("oauth-client-secret.json");
    let secret = yup_oauth2::read_application_secret(secret_path)
        .await
        .expect("oauth-client-secret.json not found!");

    let connector = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .unwrap()
        .https_only()
        .enable_http2()
        .build();

    let executor = hyper_util::rt::TokioExecutor::new();
    let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
        secret,
        yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
        yup_oauth2::client::CustomHyperClientBuilder::from(
            hyper_util::client::legacy::Client::builder(executor).build(connector),
        ),
    )
    .persist_tokens_to_disk("token-cache.json")
    .build()
    .await
    .unwrap();

    // let auth = InstalledFlowAuthenticator::builder(
    //     secret,
    //     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    // )
    // .persist_tokens_to_disk("token_cache.json")
    // .build()
    // .await
    // .expect("failed to create authenticator");

    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
        .build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .unwrap()
                .https_or_http()
                .enable_http2()
                .build(),
        );

    let hub = DriveHub::new(client, auth);

    println!("Fetching data . . .");

    let result = hub
        .files()
        .list()
        .page_size(5)
        .q("trashed = false")
        .add_scope("https://www.googleapis.com/auth/drive.readonly")
        .doit()
        .await;

    match result {
        Ok((_response, file_list)) => {
            if let Some(files) = file_list.files {
                println!("Found {} files:", files.len());
                for file in files {
                    println!(
                        "- {} ({})",
                        file.name.unwrap_or_default(),
                        file.id.unwrap_or_default()
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
