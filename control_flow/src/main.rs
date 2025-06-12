enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin { 
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5, 
        Coin::Dime => 10, 
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
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

fn add_fancy_hat() {
    println!("Add fancy hat");
}
fn remove_fancy_hat() {
    println!("Remove fancy hat");
}
fn reroll() {
    println!("Reroll");
}

fn main() {
    let penny = value_in_cents(Coin::Penny);
    let nickel = value_in_cents(Coin::Nickel);
    let dime = value_in_cents(Coin::Dime);
    let quarter_alabama = value_in_cents(Coin::Quarter(UsState::Alabama));
    let quarter_alaska = value_in_cents(Coin::Quarter(UsState::Alaska));
    
    println!("penny: {} cents", penny);
    println!("nickel: {} cents", nickel);
    println!("dime: {} cents", dime);
    println!("quarter_alabama: {} cents", quarter_alabama);
    println!("quarter_alaska: {} cents\n", quarter_alaska);
    
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    println!("five: {:?}, six: {:?}, none: {:?}\n", five, six, none);
    
    let dice_roll = 3;
    match dice_roll { 
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
    
    let config_max = Some(3u8);
    
    match config_max {
        Some(max) => println!("The config_max is {}", max),
        _ => (),
    }
    
    if let Some(max) = config_max {
        println!("The config_max is {}", max);
    }
    
    println!("The config_max is {:?}", config_max); // u8 타입이 Copy 트레이트를 구현해서 소유권 안사라짐

    let s = Some(String::from("string"));

    if let Some(ref val) = s {
        if val == "string" {
            println!("s is string");
        }
    }
    
    println!("s: {:?}", s);
}