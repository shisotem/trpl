// lib.rs: restaurant(=package名)という名前の、ライブラリクレートのクレートルート(クレートルート=>crateモジュールを暗黙的に形成)
// (https://chatgpt.com/share/681f453c-9d80-8005-845e-326658ddfb1d)

// // eat_at_restaurantとfront_of_houseは同じcrateモジュールで定義されている(兄弟)ので、pubがなくてもアクセスできる
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// // 公開API
// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();
//     // Relative path
//     front_of_house::hosting::add_to_waitlist();
//     self::front_of_house::hosting::add_to_waitlist();
// }

// fn serve_order() {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();

//         // Absolute path
//         crate::serve_order();
//         // Relative path
//         super::serve_order();
//     }

//     fn cook_order() {}
// }

// // 構造体はフィールド毎にpubをつけるか判断
// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }

//     impl Breakfast {
//         // constructor needed
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }

// pub fn eat_at_restaurant() {
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     meal.toast = String::from("Wheat");
//     // meal.seasonal_fruit = String::from("blueberries"); // err
// }

// // enumを公開するとその列挙子はすべて公開される
// mod back_of_house {
//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }
// }

// pub fn eat_at_restaurant() {
//     let order1 = back_of_house::Appetizer::Soup;
//     let order2 = back_of_house::Appetizer::Salad;
// }

// --------------

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// use crate::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
// }

// mod example {
//     // 構造体やenumなどなどはfull pathが慣例
//     use std::collections::HashMap;

//     fn example_fn() {
//         let mut map = HashMap::new();
//         map.insert(1, 2);
//     }
// }

// use std::fmt;
// use std::io;

// fn function1() -> fmt::Result {
//     // ...
// }

// fn function2() -> io::Result<()> {
//     // ...
// }

// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function1() -> Result {
//     // ...
// }

// fn function2() -> IoResult<()> {
//     // ...
// }

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// // re-exporting: あたかもここのスコープ(crate::)でhostingという名前で定義されているかのように外部に見せる
// pub use crate::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
// }

// // クレートからスコープへと持ち込む
// use rand::Rng;

// fn main() {
//     let secret_number = rand::thread_rng().gen_range(1..=100);
// }

// use std::{cmp::Ordering, io};

// use std::io::{self, Write};

// use std::collections::*;

// --------------

mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// mod mod_name;: コンパイラに対して、mod_name という名前のモジュールが
// 現在のスコープに存在することを宣言し、その定義（実装）を含む外部ファイルを
// 探索するよう指示する 役割を持ちます。コンパイラは、この宣言に基づいて
// mod_name.rs または mod_name/mod.rs を探しに行き、その内容をコンパイルします。

// use some_name;: 現在のスコープに、他のモジュールやクレートで定義された
// アイテム（関数、構造体、列挙型など）への短いパス（エイリアス）を作成する
// 役割を持ちます。これにより、フルパスを書かずに some_name だけでアイテムを
// できるようになります。
