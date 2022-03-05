#[derive(Debug)]
// 列挙型
enum IpAddrKind {
    // 列挙子
    v4,
    v6,
}

enum IpAddr {
    // v4(String),
    v4(u8, u8, u8, u8),
    v6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // メソッド本体をここに定義する
    }
}

enum Option<T> {
    Some(T),
    None,
}

fn main() {
    println!("Hello, world!");

    // enumのインスタンスの生成
    let four = IpAddrKind::v4;
    let six = IpAddrKind::v6;

    fn route(ip_type: IpAddrKind) {
        println!("{:#?}", ip_type);
    }

    route(IpAddrKind::v4);
    route(IpAddrKind::v6);

    let home = IpAddr::v4(127, 0, 0, 1);
    let loopback = IpAddr::v6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
}
