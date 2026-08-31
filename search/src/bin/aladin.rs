//! 참고: https://docs.google.com/document/d/1mX-WxuoGs8Hy-QalhHcvuV17n50uGI2Sg_GHofgiePE/edit?tab=t.0
use std::env;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let api_key = env::var("ALADIN_API_KEY").expect("환경변수 없음");
    let client = reqwest::Client::new();
    let url = "http://www.aladin.co.kr/ttb/api/ItemSearch.aspx";
    let q: [(String, String); 7] = [
        ("TTBKey".to_string(), api_key),
        ("Output".to_string(), "JS".to_string()),
        ("Query".to_string(), "투자자산운용사".to_string()),
        ("MaxResults".to_string(), "50".to_string()),
        ("Start".to_string(), "10".to_string()),
        ("OptResult".to_string(), "ebookList,usedList,fileFormatList".to_string()),
        ("Version".to_string(), "20131101".to_string()),
        //("RecentPublishFilter".to_string(), "1".to_string())
    ];

    let response = client.get(url).query(&q).send().await.expect("요청 실패");
    println!("Status code: {}", response.status().as_str());
    let response_text = response.text().await.expect("오류 발생");
    println!("{:?}", response_text);
}
