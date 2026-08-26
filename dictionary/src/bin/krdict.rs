//! 참고: https://krdict.korean.go.kr/kor/openApi/openApiInfo
use std::env;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let api_key = env::var("KRDICT_API_KEY").expect("환경변수 없음");
    let client = reqwest::Client::new();
    let url = "https://krdict.korean.go.kr/api/search";
    let q = internal::krdict::Request::build(api_key, "예사".to_string(), None, None);


    let response = client
        .get(url)
        .query(&q)
        .send()
        .await
        .expect("요청 실패");
    println!("Status code: {}", response.status().as_str());
    let response_text = response.text().await.expect("결과 실패");
    println!("{:?}", response_text);
}
