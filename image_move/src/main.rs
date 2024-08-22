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
        println!("Headers:\n{:#?}", res.headers());
        println!("Body:\n{}", res.text().await?);
        Ok(())
    }
}

async fn get_image_files(image_dir: &String) -> Result<Vec<String>, ExitFailure> {
    Ok(vec![format!("./{}/{}", image_dir, "reason_img01.jpg")])
}

#[tokio::main]
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

    let file_name = target_file_names.first().unwrap();
    let res = GyazoFile::upload(&file_name).await?;
    println!("{:?}", res);
    Ok(())
}
