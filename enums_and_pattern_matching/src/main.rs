#[derive(Debug)]
// 列挙型と構造体
// enum IpAddrKind {
//     V4,
//     V6,
// }
// 
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }
// 
// let home = IpAddr {
//     kind: IpAddrKind::V4,
//     address: String::from("127.0.0.1"),
// };
// 
// let loopback = IpAddr {
//     kind: IpAddrKind::V6,
//     address: String::from("::1"),
// };

// 列挙型のみ
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("IPv4: {:?}, IPv6: {:?}", home, loopback);
}
