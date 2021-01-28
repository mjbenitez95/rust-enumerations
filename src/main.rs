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

impl Message {
    fn call(&self) {
        println!("{:?}", self)
    }
}

fn main() {
    create_normal_enums();
    introduce_special_enums();
    introduce_options();
    play_with_options();
}

fn create_normal_enums(){
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));
}

fn introduce_special_enums(){
    let m = Message::Write(String::from("hello"));
    m.call();
}

fn introduce_options(){
    let some_number = Some(5);
    let some_string = Some("This is a string");
    let absent_number: Option<i32> = None;

    println!(
        "Some number: {:?} and some string: \"{:?}\" and absent number == some: {}.",
        some_number, some_string, absent_number == Some(5)
    );
}

fn play_with_options(){
    let x: i8 = 5;
    let y: Option<i8> = None;
    println!("y is Some: {}, y is None: {}.", y.is_some(), y.is_none());

    if y.is_some() {
        let y_value = y.unwrap();
        println!("{} plus {} is {}!", x, y_value, x+y_value);
    } else {
        println!("y contains None, and x is: {}.", x);
    }
}