# `kagi-openapi-rust` Search API 사용법

이 예제는 저장소에 포함된 공식 생성 클라이언트 `kagi-openapi-rust`로 Kagi Search API를 호출한다. 크레이트의 실제 구현을 기준으로 작성했으며, 실행 코드는 [`src/main.rs`](src/main.rs)에 있다.

## 빠른 실행

Kagi API Dashboard에서 발급한 키를 `KAGI_API_KEY` 환경 변수로 제공한다. 키를 소스 코드에 직접 적거나 로그로 출력하지 않는다.

```bash
export KAGI_API_KEY='발급받은_API_키'
cargo run -p search -- "Rust async trait"
```

프로젝트 루트나 현재 디렉터리의 `.env`에 `KAGI_API_KEY`가 있으면 `dotenvy`가 자동으로 읽는다. `.env`가 없어도 셸에 환경 변수가 설정되어 있으면 실행된다. 검색어를 생략하면 예제의 기본 검색어를 사용한다.

## 최소 호출 구조

```rust
use kagi_openapi_rust::{
    apis::{configuration::Configuration, search_api},
    models::SearchRequest,
};

let configuration = Configuration {
    bearer_access_token: Some(api_key),
    ..Configuration::default()
};

let mut request = SearchRequest::new("검색어".to_owned());
request.limit = Some(5);
request.safe_search = Some(true);

let response = search_api::search(&configuration, request).await?;
```

`search_api::search`는 비동기 함수이므로 Tokio 같은 async 런타임 안에서 호출해야 한다. 현재 예제는 `#[tokio::main]`을 사용한다.

## 가장 중요한 인증 주의점

`Configuration`에는 `basic_auth`, `api_key`, `oauth_access_token`, `bearer_access_token`이 모두 있지만, 현재 생성된 `src/apis/search_api.rs`의 `search()`는 다음 동작만 한다.

```rust
if let Some(ref token) = configuration.bearer_access_token {
    req_builder = req_builder.bearer_auth(token.to_owned());
}
```

따라서 Kagi API 키는 반드시 `bearer_access_token: Some(api_key)`로 설정해야 한다. `basic_auth`나 `api_key`에 값을 넣어도 현재 `search()` 요청에는 사용되지 않는다. `bearer_auth`가 `Bearer ` 접두사를 붙이므로 환경 변수에는 접두사 없이 키 값만 저장한다.

기본 API 주소와 HTTP 클라이언트는 `Configuration::default()`가 이미 설정한다.

- 기본 주소: `https://kagi.com/api/v1`
- 엔드포인트: `POST /search`
- 요청 본문: JSON으로 직렬화된 `SearchRequest`
- 성공 응답: `Search200Response`

## 요청 만들기

`SearchRequest::new(query)`를 사용하면 필수 필드인 `query`만 설정되고 나머지는 `None`이 된다. 구조체 리터럴로 모든 필드를 나열할 필요가 없다.

주요 선택 필드는 다음과 같다.

| 필드 | 타입/범위 | 용도 |
| --- | --- | --- |
| `workflow` | `Search`, `Images`, `Videos`, `News`, `Podcasts` | 결과 종류 선택 |
| `page` | `1..=10` | 페이지 선택 |
| `limit` | `1..=1024` | 반환되는 최대 결과 수 제한 |
| `safe_search` | `bool` | 성인 가능성이 있는 결과 제외 |
| `timeout` | `f64` 초 | 검색 결과 수집 시간 제한 |
| `lens_id` | `String` | 공개 Kagi Lens ID 또는 URL 적용 |
| `lens` | `SearchRequestLens` | 도메인, 키워드, 파일 형식, 날짜 등을 인라인으로 제한 |
| `filters` | `SearchRequestFilters` | 지역과 시작/종료 날짜 필터 |
| `extract` | `SearchRequestExtract` | 결과 페이지 본문을 추출하여 `snippet`에 저장 |
| `personalizations` | `SearchRequestPersonalizations` | 도메인·정규식 기반 순위 조정 |

워크플로를 바꾸는 예시는 다음과 같다.

```rust
use kagi_openapi_rust::models::search_request::Workflow;

let mut request = SearchRequest::new("서울 야경".to_owned());
request.workflow = Some(Workflow::Images);
request.limit = Some(10);
```

날짜와 지역 필터는 `Box`로 감싼다.

```rust
use kagi_openapi_rust::models::SearchRequestFilters;

request.filters = Some(Box::new(SearchRequestFilters {
    region: Some("KR".to_owned()),
    after: None,
    before: None,
}));
```

`extract`는 검색 API와 별도로 추가 비용이 발생할 수 있고 `count`는 `1..=10`이어야 한다. 단순 검색에는 설정하지 않는 편이 안전하다.

### `format`에 관한 생성 클라이언트 제약

`SearchRequest::format`에는 실험적인 `Format::Markdown`이 정의되어 있지만, 현재 `search()` 구현은 성공 응답을 항상 JSON `Search200Response`로 역직렬화하며 `text/plain` 응답은 오류로 처리한다. 타입이 보장되는 사용법은 `format`을 `None`으로 두거나 `Format::Json`을 쓰는 것이다.

## 응답 읽기

응답은 여러 단계가 선택 값이다.

```text
Search200Response
├── meta: Option<Meta>
└── data: Option<Box<Search200ResponseData>>
    ├── search: Option<Vec<SearchResult>>
    ├── image: Option<Vec<SearchResult>>
    ├── video: Option<Vec<SearchResult>>
    ├── news: Option<Vec<SearchResult>>
    └── 그 밖의 결과 그룹
```

기본 웹 검색 결과는 `response.data.search`, 이미지 워크플로 결과는 `response.data.image`, 뉴스 결과는 `response.data.news`에 들어간다. 각 `SearchResult`의 핵심 필드는 다음과 같다.

- `url: String`
- `title: String`
- `snippet: Option<String>`
- `time: Option<String>`
- `image: Option<Box<SearchResultImage>>`
- `props: Option<HashMap<String, serde_json::Value>>`

`props`는 결과 종류에 따라 달라지는 임의 메타데이터다. 안정적인 고정 구조로 가정하지 말고 필요한 키가 있는지 확인한 뒤 읽는다. `meta.trace`는 Kagi 지원팀에 요청 문제를 문의할 때 유용하지만, `Meta` 자체는 변경될 수 있다고 모델에 명시되어 있다.

## 오류 처리

반환 타입은 다음과 같다.

```rust
Result<Search200Response, apis::Error<search_api::SearchError>>
```

`apis::Error`의 종류는 다음과 같다.

- `Reqwest`: 요청 생성, 연결 또는 응답 수신 오류
- `Serde`: JSON 역직렬화 오류
- `Io`: I/O 오류
- `ResponseError`: HTTP 오류 상태와 응답 본문, 파싱된 `SearchError`

`SearchError`는 HTTP `400`, `401`, `403`, `429`, `500`을 각각 `ErrorEnvelope`로 제공한다. 인증 오류는 먼저 API 키가 `bearer_access_token`에 설정되었는지 확인한다. 제한 초과는 `429`, 잘못된 요청 필드는 보통 `400`으로 확인할 수 있다. 운영 환경에서 오류를 기록할 때는 API 키를 절대 포함하지 않는다.

## 소스에서 확인한 공개 API 위치

- `kagi-openapi-rust/src/apis/configuration.rs`: 설정과 인증 필드
- `kagi-openapi-rust/src/apis/search_api.rs`: 실제 요청 생성, Bearer 인증, 성공·오류 역직렬화
- `kagi-openapi-rust/src/models/search_request.rs`: 요청 필드와 `Workflow`, `Format`
- `kagi-openapi-rust/src/models/search_200_response_data.rs`: 결과 그룹
- `kagi-openapi-rust/src/models/search_result.rs`: 개별 검색 결과

생성된 전체 Rust API 문서는 워크스페이스 루트에서 다음 명령으로 만들 수 있다.

```bash
cargo doc -p kagi-openapi-rust --no-deps
```
