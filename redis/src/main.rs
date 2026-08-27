use redis::{FromRedisValue, JsonCommands, ParsingError, TypedCommands, Value};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct JsonSampleF1 {
    a: i32
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonSampleF2 {
    b: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonSample {
    f1: JsonSampleF1,
    f2: JsonSampleF2
}

impl FromRedisValue for JsonSample {
    fn from_redis_value(v: Value) -> Result<Self, ParsingError> {
        let json_str: String = redis::from_redis_value(v)?;
        serde_json::from_str(&json_str).map_err(|e| {
            ParsingError::from("aaa")
        })
    }
}

fn main() {
    // ACL SETUSER test on >password allkeys allcommands
    // ACL SETUSER default off
    let client = redis::Client::open("redis://test:password@127.0.0.1:6379").expect("Redis 연결 실패");
    let mut connection = client.get_connection().expect("Redis 열기 실패");
    // 간단 예제
    connection.set("hello", "10").expect("삽입 실패");
    connection.set("world", 10).expect("삽입 실패");
    let hello = connection.get("hello").expect("가져오기 실패");
    println!("{:?}", hello);
    let world = connection.get_int("world").expect("가져오기 실패");
    println!("{:?}", world);
    let hello = connection.get_int("hello").expect("가져오기 실패");
    println!("{:?}", hello);
    // 리스트 사용 예제
    connection.rpush("list", "first").expect("삽입 실패");
    connection.lpush("list", "zero").expect("삽입 실패");
    connection.rpush("list", "second").expect("삽입 실패");
    let first = connection.lindex("list", 1).expect("가져오기 실패");
    println!("{:?}", first);
    connection.lset("list", 1, "FIRST").expect("수정 실패");
    let first = connection.lindex("list", 1).expect("가져오기 실패");
    println!("{:?}", first);
    // hashmap 사용 예제
    connection.hset("hash", "hello", "world").expect("삽입 실패");
    connection.hset("hash", "안녕", "하세요").expect("삽입 실패");
    connection.hset("hash", "!!", "!?!?").expect("삽입 실패");
    let getall = connection.hgetall("hash").expect("가져오기 실패");
    for h in getall {
        println!("Key: {}, Value: {}", h.0, h.1);
    }

    connection.hset("sample:1", "name", "product1").expect("삽입 실패");
    connection.hset("sample:1", "price", "100").expect("삽입 실패");
    connection.hset("sample:2", "name", "product2").expect("삽입 실패");
    let getall = connection.hgetall("sample").expect("가져오기 실패");
    for h in getall {
        println!("Key: {}, Value: {}", h.0, h.1);
    }

    // json 관련 예제 - json 모듈 활성화 필요
    let json_sample = JsonSample {
        f1: JsonSampleF1 {
            a: 1
        },
        f2: JsonSampleF2 {
            b: vec!["hello".to_string(), "world".to_string()],
        },
    };
    connection.json_set::<_, _, _, ()>("thisisjson", "$", &json_sample).expect("삽입 실패");
    let json: Option<JsonSample> = connection.json_get("thisisjson", "$").expect("가져오기 실패");
    println!("JSON Get Result: {:?}", json);
}
