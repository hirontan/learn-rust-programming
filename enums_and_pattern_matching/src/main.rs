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

enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => {
            println!("Lucky nickel!");
            5
        },
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("IPv4: {:?}, IPv6: {:?}", home, loopback);

    let coin = Coin::Nickel;

    let value = value_in_cents(coin);
    println!("value: {}", value);

    let c = Coin::Quarter(UsState::Alabama);
    let value = value_in_cents(c);
    println!("value: {}", value);
}

