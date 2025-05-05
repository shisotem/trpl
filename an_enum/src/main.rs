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
}
