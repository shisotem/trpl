// lib.rs: restaurant(=package名)という名前の、ライブラリクレートのクレートルート(クレートルート=>crateモジュールを暗黙的に形成)
// (https://chatgpt.com/share/681e3458-437c-8005-930c-45683a2a787c)

mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
