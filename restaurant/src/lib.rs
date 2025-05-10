// lib.rs: restaurant(=package名)という名前の、ライブラリクレートのクレートルート(クレートルート=>crateモジュールを暗黙的に形成)
// (https://chatgpt.com/share/681e3458-437c-8005-930c-45683a2a787c)

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

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

mod example {
    // 構造体やenumなどなどはfull pathが慣例
    use std::collections::HashMap;

    fn example_fn() {
        let mut map = HashMap::new();
        map.insert(1, 2);
    }
}
