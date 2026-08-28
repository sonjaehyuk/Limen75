fn main() {
    let client = memcache::connect("memcache://127.0.0.1:11211?timeout=10&tcp_nodelay=true").unwrap();
    client.flush().unwrap();
    // 값 저장하고 조회하기
    client.set("foo", "bar", 0).unwrap();
    let value: Option<String> = client.get("foo").unwrap();
    println!("{:?}", value);
    // 값 수정하기
    client.prepend("foo", "foo").unwrap();
    client.append("foo", "baz").unwrap();
    let value: Option<String> = client.get("foo").unwrap();
    println!("{:?}", value);
    // 삭제하기
    client.delete("foo").unwrap();

    // 숫자
    client.set("counter", 40, 0).unwrap();
    client.increment("counter", 2).unwrap();
    let answer: i32 = client.get("counter").unwrap().unwrap();
    println!("{answer}")

}
