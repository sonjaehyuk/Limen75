#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    // let api_key = env::var("KAGI_API_KEY").expect("환경변수 없음")
}
