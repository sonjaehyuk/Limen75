//! 참고: https://developers.kakao.com/docs/ko/daum-search/common
use std::env;
use serde::Deserialize;
use chrono::prelude::*;
use reqwest::header::{HeaderMap, HeaderValue};

#[derive(Deserialize, Debug)]
struct DaumSearchResponseMeta {
    total_count: u32,
    pageable_count: u32,
    is_end: bool,
}

#[derive(Deserialize, Debug)]
struct DaumSearchResponseDocument {
    title: String,
    contents: String,
    url: String,
    datetime: DateTime<Local>,
}

#[derive(Deserialize, Debug)]
struct DaumSearchResponse {
    meta: DaumSearchResponseMeta,
    documents: Vec<DaumSearchResponseDocument>
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let api_key = env::var("KAKAO_REST_API_KEY").expect("환경변수 없음");
    let auth_header = format!("KakaoAK {}", api_key);
    let mut h = HeaderMap::new();
    h.insert(reqwest::header::AUTHORIZATION, HeaderValue::from_str(&auth_header).unwrap());
    let client = reqwest::Client::new();
    let url = "https://dapi.kakao.com/v2/search/web";
    let q: [(String, String); 1] = [("query".to_string(), "한국외국어대학교".to_string())];

    let response = client
        .get(url)
        .headers(h)
        .query(&q)
        .send()
        .await
        .expect("요청 실패");
    println!("Status code: {}", response.status().as_str());
    let daum_response: DaumSearchResponse = response.json().await.expect("json 실패");
    println!("{:?}", daum_response)
}
