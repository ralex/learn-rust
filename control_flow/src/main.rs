enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska,
    // shorted list...
    Wisconsin,
    Wyoming,
}

fn value_in_cents(coin: Coin) -> u8 {
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

fn main() {
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Nickel);
    value_in_cents(Coin::Dime);
    value_in_cents(Coin::Quarter(UsState::Wisconsin));

    fn plus_one(x: Option<i32>) -> Option<i32> {
        x.map(|i| i + 1)
    }

    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);


    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() { println!("Add fancy hat!") }
    fn remove_fancy_hat() { println!("Remove fancy hat!") }


    // let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {}", max),
    //     _ => (),
    // }

    // if let Some(max) = config_max {
    //     println!("The maximum is configured to be {}", max);
    // }
    // More concise way using map
    if let Some(config_max) = Some(3u8) { println!("The maximum is configured to be {:?}", config_max) }
}
