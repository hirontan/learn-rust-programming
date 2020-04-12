
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
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Test,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    let mut count = 0;
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
        },
        _ => {
            count += 1;
            count
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

    // let c2 = Coin::Quarter("test");
    let c2 = Coin::Test;
    let value2 = value_in_cents(c2);
    println!("value: {}", value2);
}

