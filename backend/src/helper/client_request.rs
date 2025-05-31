use reqwest::{multipart, Client};
use serde_json::Value;
use crate::model::photo_data_model::CollectedField;
use dotenvy::dotenv;
use std::env;

pub async fn save_target_data(
    name: &str,
    files: Vec<CollectedField>,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {

    dotenv().ok();
    let add_target_url = env::var("ADD_TARGET_URL")
        .expect("ADD_TARGET_URL must be set in .env file");
    let client = Client::new();

    tracing::info!("Saving target data to: {}", add_target_url);

    let mut form = multipart::Form::new()
        .text("name", name.to_string());

    for file in files {
        let file_bytes = file.data.to_vec(); // Convert Bytes to Vec<u8>
        let file_name = file.file_name.unwrap_or("unknown.jpg".into());
        let mime = file
            .content_type
            .map(|ct| ct.to_string())
            .unwrap_or_else(|| "application/octet-stream".to_string());

        let part = multipart::Part::bytes(file_bytes)
            .file_name(file_name)
            .mime_str(&mime)?;

        form = form.part("files", part);
    }

    let res = client
        .post(add_target_url)
        .multipart(form)
        .send()
        .await?;

    let data = res.json::<serde_json::Value>().await?;
    tracing::info!("Response from server: {:?}", data);

    Ok(data)
}


pub async fn recognize_target(
    file: CollectedField,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync + 'static>> {
    dotenv().ok();

    let recognize_target_url = env::var("RECOGNIZE_TARGET_URL")
        .expect("RECOGNIZE_TARGET_URL must be set in .env file");

    let client = Client::new();

    let file_bytes = file.data.to_vec(); // Convert Bytes to Vec<u8>
    let file_name = file.file_name.unwrap_or_else(|| "unknown.jpg".to_string());
    let mime = file
        .content_type
        .map(|ct| ct.to_string())
        .unwrap_or_else(|| "application/octet-stream".to_string());

    let part = multipart::Part::bytes(file_bytes)
        .file_name(file_name)
        .mime_str(&mime)?;

    let form = multipart::Form::new().part("file", part);

    let res = client
        .post(recognize_target_url)
        .multipart(form)
        .send()
        .await?;

    let data = res.json::<serde_json::Value>().await?;
    tracing::info!("Response from server: {:?}", data);
    Ok(data)
}