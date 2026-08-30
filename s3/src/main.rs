use opendal::{services::S3, Operator};
use std::env;
use tokio::io::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env 불러오기 실패");
    let endpoint = env::var("AWS_ENDPOINT_URL").expect("환경변수 실패");
    let region = env::var("AWS_REGION")
        .unwrap_or_else(|_| "auto".to_string());
    let bucket = String::from("sonjaehyuk");

    let access_key = env::var("AWS_ACCESS_KEY_ID").expect("환경변수 실패");
    let secret_key = env::var("AWS_SECRET_ACCESS_KEY").expect("환경변수 실패");

    let builder = S3::default()
        .root("/")
        .bucket(&bucket)
        .endpoint(&endpoint)
        .region(&region)
        .access_key_id(&access_key)
        .secret_access_key(&secret_key);
    let storage = Operator::new(builder).expect("s3 실패");
    let file = tokio::fs::read("s3/20260530_145848.jpg").await.expect("파일 읽기 실패");
    storage.write("bookimage.png", file).await.expect("파일 올리기 실패");
    if let Ok(a) = storage.exists("bookimage.png").await {
        println!("이미지 존재 여부: {a}");
    }
    let data = storage.read("bookimage.png").await.expect("파일 읽기 실패");
    println!("downloaded {} bytes", data.len());

    // 파일 나눠서 올리기
    let mut writer = storage
        .writer_with("bookimage2.png")
        .chunk(8 * 1024 * 1024)
        .concurrent(4)
        .await.expect("writer 생성 실패");

    let mut file = tokio::fs::File::open("s3/20260530_145848.jpg").await.expect("파일 열기 실패");
    let mut buffer = vec![0u8; 8 * 1024];

    loop {
        let n = file.read(&mut buffer).await.expect("읽기 실패");

        if n == 0 {
            break;
        }

        writer.write(buffer[..n].to_vec()).await.expect("올리기 실패");
    }
    writer.close().await.expect("writer 닫기 실패");
    
}
