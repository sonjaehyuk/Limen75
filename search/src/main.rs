use std::{env, error::Error, io};

use kagi_openapi_rust::{
    apis::{configuration::Configuration, search_api},
    models::SearchRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();

    let api_key = env::var("KAGI_API_KEY").map_err(|_| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "KAGI_API_KEY 환경 변수가 설정되지 않았습니다",
        )
    })?;

    // search_api::search의 구현은 이 필드만 Bearer 인증에 사용합니다.
    let configuration = Configuration {
        bearer_access_token: Some(api_key),
        ..Configuration::default()
    };

    let mut request = SearchRequest::new("한국외국어대학교".to_string());
    request.limit = Some(5);
    request.safe_search = Some(true);

    let response = search_api::search(&configuration, request).await?;

    if let Some(meta) = response.meta {
        println!(
            "(처리 시간: {} ms, trace: {})",
            meta.ms
                .map_or_else(|| "알 수 없음".to_owned(), |ms| ms.to_string()),
            meta.trace.as_deref().unwrap_or("없음")
        );
    } else {
        println!("검색 완료");
    }

    let results = response
        .data
        .and_then(|data| data.search)
        .unwrap_or_default();

    if results.is_empty() {
        println!("일반 웹 검색 결과가 없습니다.");
        return Ok(());
    }

    for (index, result) in results.iter().enumerate() {
        println!("\n{}. {}\n   {}", index + 1, result.title, result.url);
        if let Some(snippet) = &result.snippet {
            println!("   {snippet}");
        }
    }

    Ok(())
}
