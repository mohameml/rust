#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

pub fn enum_() {
    let localhost = IpAddrKind::V4(127, 0, 0, 1);

    let m = Message::Move { x: 1, y: 2 };

    println!("m = {m:?}");

    println!("localhost = {localhost:?}");
}

fn route(ip_kind: IpAddrKind) {}
