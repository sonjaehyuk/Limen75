# Kagi 검색 API 관련

Kagi API는 Search와 Extract API를 제공하고 있다. 아래 설명은 Kagi 생태계를 이해하고 사용해본 경험이 있다는 것을 전제로 한다. 

## Search

Kagi에서 제공하는 가장 기본 검색엔진 API이다.

| 이름   | 설명                           |
|--------|--------------------------------|
| 경로   | https://kagi.com/api/v1/search |
| 메서드 | POST                           |

### 요청 본문 구성

* query: (필수) 검색어
* workflow: 어떤 유형을 반환할지 결정. 가능한 값: "search"(기본), "images", "videos", "news", "podcasts"
* format: API 응답을 어떻게 직렬화해서 보낼지 결정. 가능한 값: "json"(기본), "markdown"(실험)
* lens_id: Kagi에서 쓰는 렌즈 id 값. [help.kagi.com/kagi/features/lenses.html](https://help.kagi.com/kagi/features/lenses.html)에서 렌즈가 무엇인지 학습할 수 있다. 렌즈의 id 값을 문자열로 넣어보내면 된다.
* ~~lens~~: 개별 요청마다 렌즈를 만들어서 보낼 수 있다. **그러나 `lens_id`처럼 실제로 렌즈가 유의미하게 작동하는 것을 관찰하기 어렵다**.
* timeout: 검색 시간 초과를 설정한다. 0.5부터 4까지의 숫자 중에서 선택할 수 있다. 생략하면 Kagi의 기본 시간 초과가 적용된다.
* page: 검색 결과 페이지를 1부터 10 사이에서 선택할 수 있다. 
* limit: 돌아올 결과의 최대 수. 1과 1024 사이여야 한다. 이것은 요청된 결과의 양을 변경하지 않으며 반환된 최대 금액만 제한되는 설정이다. 생략하면 API는 항상 얻을 수있는 가장 많은 결과를 제공한다.
* filters: 검색 결과를 정제하는 부가 기능을 설정한다.
  * region: **소문자로** 지역 코드를 설정하여 특정 지역 결과만 반환하게 할 수 있다. 지역 코드는 https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2#Officially_assigned_code_elements 참고. 
  * after: 날짜를 지정하여 그 날짜 이후의 결과만을 반환하게 할 수 있다.
  * before: 날짜를 지정하여 그 날짜 이전의 결과만을 반환하게 할 수 있다.
* safe_search: 안전 검색을 설정할 수 있다. 불리언으로 설정한다. 기본으로 `true`가 설정되어 있다.
* personalizations: 검색 결과에서 순위를 지정하여 결과를 반환하게 할 수 있게 해준다.
  * domains: 도메인 단위로 순위를 지정하는 객체의 배열이다. 최대 길이는 1000이다.
    * domain 속성: 도메인 이름이다. tld만 표시해도 된다.
    * kind 속성: 순위를 어떻게 할지 결정한다. 이중에서 선택해야 한다: "block", "lower", "raise", "pin"
