use dotenv::dotenv;
use exitfailure::ExitFailure;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::Client;
use serde_derive::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct MetadataRaw {
    app: Option<String>,
    title: Option<String>,
    url: Option<String>,
    desc: String,
    original_title: Option<String>,
    original_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Metadata {
    app: String,
    title: String,
    url: String,
    desc: String,
    original_title: String,
    original_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct GyazoImageQuoteRaw {
    image_id: String,
    permalink_url: String,
    url: String,
    metadata: MetadataRaw,
    thumb_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct GyazoImageQuote {
    image_id: String,
    permalink_url: String,
    metadata: Metadata,
    url: String,
    thumb_url: String,
}

impl GyazoImageQuote {
    fn from_raw(raw: &GyazoImageQuoteRaw) -> GyazoImageQuote {
        GyazoImageQuote {
            image_id: raw.image_id.clone(),
            permalink_url: raw.permalink_url.clone(),
            url: raw.url.clone(),
            thumb_url: raw.thumb_url.clone(),
            metadata: Metadata {
                app: raw.metadata.app.clone().unwrap_or_else(|| "".to_string()),
                title: raw.metadata.title.clone().unwrap_or_else(|| "".to_string()),
                url: raw.metadata.url.clone().unwrap_or_else(|| "".to_string()),
                desc: raw.metadata.desc.clone(),
                original_title: raw
                    .metadata
                    .original_title
                    .clone()
                    .unwrap_or_else(|| "".to_string()),
                original_url: raw
                    .metadata
                    .original_url
                    .clone()
                    .unwrap_or_else(|| "".to_string()),
            },
        }
    }

    async fn get(api_key: &String) -> Result<Vec<GyazoImageQuote>, ExitFailure> {
        let url = format!("https://api.gyazo.com/api/images");
        let client = Client::new();
        let header_str = format!("Bearer {}", api_key);
        let mut headers = HeaderMap::new();

        headers.insert(AUTHORIZATION, HeaderValue::from_str(&*header_str)?);

        let res = client
            .get(&url)
            .headers(headers)
            .send()
            .await?
            .json::<Vec<GyazoImageQuoteRaw>>()
            .await;

        let images: Vec<GyazoImageQuoteRaw> = match res {
            Ok(imgs) => imgs,
            Err(e) => {
                eprintln!("Error parsing JSON: {}", e);
                vec![]
            }
        };

        let quatation: Vec<GyazoImageQuote> = images
            .iter()
            .map(|raw| GyazoImageQuote::from_raw(raw))
            .collect();
        Ok(quatation)
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    dotenv().ok();

    // 環境変数を取得
    let gyazo_api_token = env::var("GYAZO_APP_TOKEN").expect("GYAZO_APP_TOKEN must be set");
    let res = GyazoImageQuote::get(&gyazo_api_token).await?;
    println!("{:?}", res);
    Ok(())
}
