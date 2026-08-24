use std::{env, error::Error, io};
use kagi_openapi_rust::{
    apis::{configuration::Configuration, search_api},
    models::SearchRequest, models::SearchRequestFilters,
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

    let mut request = SearchRequest::new("투자자산운용사".to_string());
    //request.limit = Some(5);
    //request.safe_search = Some(true);
    request.lens_id = Some("TT8mzzN1G5jHaV9ih0lH6SlMSbru5RGW".to_string());
    request.filters = Some(Box::new(SearchRequestFilters {
        region: Some("kr".to_string()),
        after: None,
        before: None,
    }));
    request.page = Some(2);
    //let sites_included = Some(vec!["*.hufs.ac.kr".to_string()]);
    //let sites_excluded = Some(vec!["*.namu.wiki".to_string()]);
    // request.lens = Some(Box::new(SearchRequestLens {
    //     sites_included: None,
    //     sites_excluded: None,
    //     keywords_included: None,
    //     keywords_excluded: None,
    //     file_type: Some("pdf".to_string()),
    //     time_after: None,
    //     time_before: None,
    //     time_relative: None,
    //     search_region: None,
    // }));

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
