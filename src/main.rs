mod google_drive;

use google_drive3::{DriveHub, hyper_util::client::legacy::connect::HttpConnector};
use hyper_rustls::HttpsConnector;
use image::ImageReader;
use std::{fs::File, path::Path};

#[tokio::main]
async fn main() {
    let hub = google_drive::init_google_drive().await;
    upload_file(&hub).await;
}

async fn upload_file(hub: &DriveHub<HttpsConnector<HttpConnector>>) {
    println!("Please enter the path of the file you want to upload: ");
    let mut file_path_input = String::new();
    std::io::stdin()
        .read_line(&mut file_path_input)
        .expect("Failed to read the file path input!");
    let file_path = file_path_input.trim().trim_matches('"').trim_matches('\'');
    println!("your given file path: {}", &file_path);

    let mime_type = mime_guess::from_path(file_path).first().unwrap();

    println!("Change file name or skip by clicking 'Enter': ");
    let mut file_name_input = String::new();
    std::io::stdin()
        .read_line(&mut file_name_input)
        .expect("Failed to read input!");
    let file_name: Option<&str> = Some(file_name_input.trim());

    let uploaded_file_name: &str = match file_name.as_deref() {
        // If it is None, or Some(""), use the file_path
        None | Some("") => Path::new(file_path)
            .file_name()
            .and_then(|name| name.to_str()) // Convert OsStr to &str
            .unwrap_or("default_file.bin"), // Handle cases where path is invalid

        // Otherwise, use the provided value
        Some(value) => value,
    };
    println!("uploaded file name: {}", uploaded_file_name);

    // get data bytes from selected file
    let data_bytes = File::open(file_path).expect("Failed to open file!");

    // upload file to drive
    let new_file = google_drive3::api::File {
        name: Some(uploaded_file_name.to_string()),
        ..Default::default()
    };

    let result = hub
        .files()
        .create(new_file)
        .upload(data_bytes, mime_type)
        .await
        .expect("Failed to upload file to Drive!")
        .1;

    println!("File uploaded successfully~ New file ID: {:?}", result.id);
}

fn generate_fractal_image() {
    let imgx = 800;
    let imgy = 800;

    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    // A redundant loop to demonstrate reading image data
    for x in 0..imgx {
        for y in 0..imgy {
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;

            let c = num_complex::Complex::new(-0.4, 0.6);
            let mut z = num_complex::Complex::new(cx, cy);

            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }

            let pixel = imgbuf.get_pixel_mut(x, y);
            let image::Rgb(data) = *pixel;
            *pixel = image::Rgb([data[0], i as u8, data[2]]);
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("fractal.png").unwrap();
}

fn clone_image(file_path: &str, file_name: String) {
    //decode image using ImageReader from image crate
    let img = ImageReader::open(&file_path)
        .expect("Failed to open file!")
        .decode()
        .unwrap();

    // saving image locally
    img.save(format!("{}.png", file_name))
        .expect("Failed to save image!");
}
