fn main() {
    let four = IpAddrKind::V4(String::from("127.0.0.1"));
    let six = IpAddrKind::V6(String::from(".01"));
}

fn route(ip_kind: IpAddrKind) {}

enum IpAddrKind {
    V4(String),
    V6(String),
}
