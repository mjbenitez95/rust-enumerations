fn main() {
    create_normal_enums();
    println!();
    introduce_special_enums();
    println!();
    introduce_options();
    println!();
    play_with_options();
    println!();

    match_operator();
    println!();
    match_with_options();
    println!();
    if_let();
    println!();
}

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn create_normal_enums() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));
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

fn introduce_special_enums() {
    let m = Message::Write(String::from("hello"));
    m.call();
}

fn introduce_options() {
    let some_number = Some(5);
    let some_string = Some("This is a string");
    let absent_number: Option<i32> = None;

    println!(
        "Some number: {:?} and some string: \"{:?}\" and absent number == some: {}.",
        some_number,
        some_string,
        absent_number == Some(5)
    );
}

fn play_with_options() {
    let x: i8 = 5;
    let y: Option<i8> = None;
    println!("y is Some: {}, y is None: {}.", y.is_some(), y.is_none());

    if y.is_some() {
        let y_value = y.unwrap();
        println!("{} plus {} is {}!", x, y_value, x + y_value);
    } else {
        println!("y contains None, and x is: {}.", x);
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    California,
    Oregon,
    Washington
    // -- snip
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn match_operator() {
    let coin = Coin::Dime;
    println!("We have a dime! It's worth {} cents!", value_in_cents(&coin));

    let quarter = Coin::Quarter(UsState::California);
    println!("We have a quarter! It's worth {} cents!", value_in_cents(&quarter));
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn match_with_options(){
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    if(six.is_some()){
        println!("The value of plus_one(five) is: {}!", six.unwrap());
    }

    for value in (1..8) {
        match value {
            1 => println!("One!"),
            3 => println!("Three!"),
            5 => println!("Five!"),
            7 => println!("Seven!"),
            _ => println!("Something else!")
        }
    }
}