use dotenv::dotenv;
use exitfailure::ExitFailure;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::multipart::Part;
use reqwest::Client;
use serde_derive::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct GyazoFile {
    id: String,
    permalink_url: String,
    thumb_url: String,
    url: String,
}

impl GyazoFile {
    /** upload処理の実行 */
    async fn upload(target_file_name: &String) -> Result<(), ExitFailure> {
        println!("Uploading file: {}", target_file_name);

        let url = format!("https://upload.gyazo.com/api/upload");
        let gyazo_api_token = env::var("GYAZO_APP_TOKEN").expect("GYAZO_APP_TOKEN must be set");

        let client = Client::new();
        let header_str = format!("Bearer {}", gyazo_api_token);
        let mut headers = HeaderMap::new();

        headers.insert(AUTHORIZATION, HeaderValue::from_str(&*header_str)?);

        let content: Vec<u8> = tokio::fs::read(target_file_name).await?;

        let part = Part::bytes(content).file_name(target_file_name.clone());
        let file = reqwest::multipart::Form::new().part("imagedata", part);

        let res = client
            .post(&url)
            .headers(headers)
            .multipart(file)
            .send()
            .await?;

        println!("Status: {}", res.status());
        println!("Upload Finished: {}", target_file_name);
        Ok(())
    }
}

async fn get_image_files(image_dir: &String) -> Result<Vec<String>, ExitFailure> {
    let image_vec: Vec<String> = vec!["reason_img01.jpg", "beach.jpg", "image.png"]
        .iter()
        .map(|filename| format!("{}/{}", image_dir, filename))
        .collect();

    Ok(image_vec)
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<(), ExitFailure> {
    dotenv().ok();

    let args: Vec<String> = env::args().collect();

    let mut image_dir: String = "imgs".to_string();

    if args.len() < 2 {
        println!("Since you didn't specify a company symbol, it will search current path.");
    } else {
        image_dir = args[1].clone();
    }

    let target_file_names = get_image_files(&image_dir).await?;

    let handles: Vec<_> = target_file_names
        .iter()
        .map(|file_name| {
            let file_name = file_name.clone();
            tokio::spawn(async move {
                match GyazoFile::upload(&file_name).await {
                    Ok(res) => println!("{:?}", res),
                    Err(e) => eprintln!("Error uploading file: {:?}", e),
                }
            })
        })
        .collect();

    for handle in handles {
        if let Err(e) = handle.await {
            eprintln!("Task failed: {:?}", e);
        }
    }

    Ok(())
}
