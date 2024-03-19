//use std::net::{IpAddr};

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move {x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

// Instead of Null, Rust uses Option enum that is either None or Some
fn _value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
           println!("State quarter from {:?}!", state); 
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let _home = IpAddrKind::V4(127, 0, 0, 1);

    let _loopback = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let _some_number = Some(5);
    let _some_char = Some('e');
    // must annotate if None, since the compiler cannot infer the type
    let _absent_number: Option<i32> = None;

    // Why is Option better than Null?
    // Option<i8> and i8 are different types
    // Option<i8> may not always have a valid value, so secure!
    // Must convert Option<T> to T before performing operations

    // 6.2 match control flow construct
    // every Some match must have a None as well
    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    // matching is exhaustive
    // for match enums, we can add an other arm (catch all pattern)
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
        // if we dont want to use other/ignore other values, 
        // we can use _ => reroll() or _ => (), does not bind to variable
    }

    // 6.3 if let
    // basically a match but with one arm
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // same as this 
    let config_max = Some(3u8);
    if let Some(max) = config_max { 
        println!("The maximum is configured to be {}", max);
    }

    let mut count = 0;
    let coin = Coin::Nickel;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    let coin = Coin::Nickel;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("state quarter from {:?}!", state);
    } else {
        count += 1;
    }
}


fn _route(_ip_kind: IpAddrKind) {}
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(_num_spaces: u8) {}
fn _reroll() {}