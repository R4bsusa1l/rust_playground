enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
}

//-.-.-.-.-.-.-.-.-.-.-.-.-

enum IpAddr {
    V4(String),
    V6(String),
}

fn instance(){
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
}

//-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-.-

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn instance1 (){
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}

fn options(){
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}

//-.-.-.-.-.-.-.-.-.-.-.-.-.-.-

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}