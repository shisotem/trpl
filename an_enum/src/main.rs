// struct QuitMessage; // ユニット構造体
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // タプル構造体
// struct ChangeColorMessage(i32, i32, i32); // タプル構造体

enum Message {
    Quit,
    Move { x: i32, y: i32 }, // 匿名構造体を含む列挙子
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        // Enum: メソッド定義可能
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();

    // Option::省略
    let some_number = Some(5);
    let some_string = Some("something");
    let absent_number: Option<i32> = None;

    // // Option<T> Enum: 他言語のnullよりも優れたしくみ
    // T: Rustではnullの可能性が皆無であることが保証される。自信を持って進める
    // Option<T>: nullの可能性がある場合はこちらにTを保持するようにする。Option<T>からTを取り出すことを課す (match)
}
