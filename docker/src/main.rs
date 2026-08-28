use bollard::Docker;

#[tokio::main]
async fn main() {
    let podman = Docker::connect_with_podman_defaults().expect("podman 연결 실패");
    let images = &podman
        .list_images(
            Some(bollard::query_parameters::ListImagesOptionsBuilder::default().all(true).build())
        )
        .await
        .expect("컨테이너 목록 확인 실패");
    for image in images {
        println!("{:?}", image);
    }
}
