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

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();

        // Absolute path
        crate::serve_order();
        // Relative path
        super::serve_order();
    }

    fn cook_order() {}
}
