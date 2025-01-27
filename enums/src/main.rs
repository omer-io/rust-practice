enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// let some_number = Some(5);
// let some_char = Some('e');

// let absent_number: Option<i32> = None;

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


fn main() {

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    let config_max = Some(70u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}