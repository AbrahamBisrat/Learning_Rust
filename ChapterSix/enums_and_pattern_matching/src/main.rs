#![allow(dead_code)]
// Enums and pattern matching

#[derive(Debug)]
enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(u8, u8, u8, u8),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage; // unit struct
struct MoveMessage;

fn main() {
    println!("Enums and Pattern matching...");

    let four = IpAddress::V4(127, 0, 0, 1);
    let six = IpAddress::V6(0, 0, 0, 1);

    enum_printer(four);
    enum_printer(six);

    //let first_message = enum Message {

    // Optionals - lol Java habits
    let some_number = Some(5);
    let another_number: Option<i32> = None;

    println!("{:?}", some_number.unwrap());
    //println!("{:?}", another_number.unwrap());
}

fn enum_printer(ip_address: IpAddress) {
    println!("{:?}", ip_address);
}
