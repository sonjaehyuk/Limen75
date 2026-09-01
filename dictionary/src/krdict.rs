use nutype::nutype;

#[nutype(
    validate(
        not_empty,
        len_char_max = 32,
        len_char_min = 32,
    ),
    derive(
        Debug, Clone,
    ),
)]
/// 인증키입니다. 16진수 32자리로 이루어진 String입니다.
struct Key(String);

#[nutype(
    sanitize(trim),
    validate(
        not_empty,
    ),
    derive(
        Debug, Clone,
    ),
)]
/// 검색어입니다.
struct Query(String);

#[nutype(
    validate(
        greater_or_equal = 1,
        less_or_equal = 1000
    ),
    derive(
        Debug, Clone, PartialEq, Eq,
    ),
)]
/// 검색의 시작 번호입니다. 기본값은 1이고, 최대 1000까지 설정할 수 있습니다.
struct Start(u16);

#[nutype(
    validate(
        greater_or_equal = 10,
        less_or_equal = 100
    ),
    derive(
        Debug, Clone, PartialEq, Eq,
    ),
)]
/// 검색의 시작 번호입니다. 기본값은 10입이고, 최대 100까지 설정할 수 있습니다.
struct Num(u8);

/// 정렬 방식입니다.
/// * [Sort::Dict]: 사전 순
/// * [Sort::Poplar]: 많이 찾은 순
enum Sort {
    Dict,
    Popular,
}

#[derive(strum::Display)]
/// 검색  대상입니다.
/// * [Part::Word]: 어휘. 기본값
/// * [Part::Ip]: 관용구, 속담
/// * [Part::Dfn]: 뜻풀이
/// * [Part::Exam]: 용례
enum Part {
    #[strum(to_string = "word")]
    Word,
    #[strum(to_string = "ip")]
    Ip,
    #[strum(to_string = "dfn")]
    Dfn,
    #[strum(to_string = "exam")]
    Exam,
}

#[derive(strum::Display)]
/// 다국어 번역 여부입니다.
/// * [Translated::Y]: 번역 언어를 설정할 수 있게 됩니다([TransLang]).
/// * [Translated::N]: 기본값
enum Translated {
    #[strum(to_string = "y")]
    Y,
    #[strum(to_string = "n")]
    N
}


#[derive(Clone, Debug, Eq, PartialEq, strum::Display)]
enum TransLang {
    #[strum(to_string = "0")]
    All,
    #[strum(to_string = "1")]
    English,
    #[strum(to_string = "2")]
    Japanese,
    #[strum(to_string = "3")]
    French,
    #[strum(to_string = "4")]
    Spanish,
    #[strum(to_string = "5")]
    Arabic,
    #[strum(to_string = "6")]
    Mongolian,
    #[strum(to_string = "7")]
    Vietnamese,
    #[strum(to_string = "8")]
    Thai,
    #[strum(to_string = "9")]
    Indonesian,
    #[strum(to_string = "10")]
    Russian,
    #[strum(to_string = "11")]
    Chinese
}

impl TryFrom<u8> for TransLang {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TransLang::All),
            1 => Ok(TransLang::English),
            2 => Ok(TransLang::Japanese),
            3 => Ok(TransLang::French),
            4 => Ok(TransLang::Spanish),
            5 => Ok(TransLang::Arabic),
            6 => Ok(TransLang::Mongolian),
            7 => Ok(TransLang::Vietnamese),
            8 => Ok(TransLang::Thai),
            9 => Ok(TransLang::Indonesian),
            10 => Ok(TransLang::Russian),
            11 => Ok(TransLang::Chinese),
            _ => Err(())
        }
    }
}

macro_rules! impl_try_from_ints {
    // 기본 정수 타입 전체에 적용
    ($enum:ty) => {
        impl_try_from_ints!(
            $enum,
            u16, u32, u64, u128, usize,
            i8, i16, i32, i64, i128, isize
        );
    };
    // 특정 타입만 지정
    ($enum:ty, $($num:ty),+ $(,)?) => {
        $(
            impl TryFrom<$num> for $enum {
                type Error = <$enum as TryFrom<u8>>::Error;

                fn try_from(value: $num) -> Result<Self, Self::Error> {
                    match u8::try_from(value) {
                        Ok(v) => <$enum>::try_from(v),
                        Err(_) => Err(()),
                    }
                }
            }
        )+
    };
}

impl_try_from_ints!(TransLang);

#[derive(strum::Display)]
/// 자세히 찾기 여부입니다.
/// * [Advanced::Y]: 아래 기능이 사용 가능해집니다.
/// * [Advanced::N]: 기본값
enum Advanced {
    #[strum(to_string = "y")]
    Y,
    #[strum(to_string = "n")]
    N,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Target {
    Headword,
    Definition,
    Example,
    OriginalLanguage(Lang),
    Pronunciation,
    Conjugation,
    AbbreviationOfConjugation,
    Idiom,
    Proverb,
    ReferenceInformation,
}

impl Default for Target {
    fn default() -> Self {
        Target::Headword
    }
}

impl TryFrom<u8> for Target {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Target::Headword),
            2 => Ok(Target::Definition),
            3 => Ok(Target::Example),
            4 => Ok(Target::OriginalLanguage(Lang::default())),
            5 => Ok(Target::Pronunciation),
            6 => Ok(Target::Conjugation),
            7 => Ok(Target::AbbreviationOfConjugation),
            8 => Ok(Target::Idiom),
            9 => Ok(Target::Proverb),
            10 => Ok(Target::ReferenceInformation),
            _ => Err(()),
        }
    }
}

impl_try_from_ints!(Target);

#[derive(Clone, Debug, Eq, PartialEq)]
enum Lang {
    All,
    PureKorean,
    Hanja,
    Undisclosed,
    English,
    Greek,
    Dutch,
    Norwegian,
    German,
    Latin,
    Russian,
    Romanian,
    Maori,
    Malay,
    Mongolian,
    Basque,
    Burmese,
    Vietnamese,
    Bulgarian,
    Sanskrit,
    SerboCroatian,
    Swahili,
    Swedish,
    Arabic,
    Irish,
    Spanish,
    Uzbek,
    Ukrainian,
    Italian,
    Indonesian,
    Japanese,
    Chinese,
    Czech,
    Cambodian,
    Quechua,
    Tagalog,
    Thai,
    Turkish,
    Tibetan,
    Persian,
    Portuguese,
    Polish,
    French,
    Provencal,
    Finnish,
    Hungarian,
    Hebrew,
    Hindi,
    Others,
    Danish,
}

impl Default for Lang {
    fn default() -> Self {
        Lang::All
    }
}



pub struct Request {
    key: Key,
    q: Query,
    start: Option<Start>,
    num: Option<Num>,
}

impl Request {
    pub fn build(key: String, query: String, start: Option<u16>, num: Option<u8>) -> Vec<(String, String)> {
        let key = Key::try_new(key).expect("key 변환 실패");
        let query = Query::try_new(query).expect("query 변환 실패");
        let start: Option<Start> = match start {
            None => None,
            Some(s) => Some(Start::try_new(s).expect("변환 실패"))
        };
        let num: Option<Num> = match num {
            None => None,
            Some(n) => Some(Num::try_new(n).expect("변환 실패"))
        };
        let mut v: Vec<(String, String)> = Vec::new();
        v.push(("key".to_string(), key.into_inner()));
        v.push(("q".to_string(), query.into_inner()));
        v
    }
}