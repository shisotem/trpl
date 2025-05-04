struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// tuple構造体
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,    // 省略記法
        username, // 省略記法
        active: true,
        sign_in_count: 1,
    }
}

// ユニット様構造体 - トレイト
// 構造体のフィールドに参照をもたせる場合 - ライフタイム指定子
