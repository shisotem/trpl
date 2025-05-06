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

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_value = 0u8; // 0-255
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // _: どんな値にもマッチする
    }

    // // if let: 値が一つのパターンにマッチした時にコードを走らせ、他は無視するmatchへの糖衣構文
    // // e.g. Some(3)のときのみ処理をしたい
    let some_u8_value = Some(0u8);
    // match some_u8_value {
    //     Some(3) => println!("three"),
    //     _ => (),
    // }
    if let Some(3) = some_u8_value {
        println!("three");
    }

    let mut count = 0;
    let coin2 = Coin::Quarter(UsState::Alabama);
    // match coin2 {
    //     // {:?}州のクォーターコイン
    //     Coin::Quarter(state) => println!("State quarter from {:?}!!!", state),
    //     _ => count += 1,
    // }
    if let Coin::Quarter(state) = coin2 {
        println!("State quarter from {:?}!!!", state);
    } else {
        count += 1;
    }
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

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
