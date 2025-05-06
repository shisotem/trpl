#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin1 = Coin::Quarter(UsState::Alaska);
    let coin1_in_cent = value_in_cents(coin1); // State quarter from Alaska!
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // アームは式(e.g. ブロック式)
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
