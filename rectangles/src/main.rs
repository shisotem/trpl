#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // self(ただし稀), &self, &mut selfを使い分ける
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area() // 借用 => mainは所有権を保ち、 rect1を使用し続けることができる
    );

    // // println!("rect1 is {}", rect1); // {}: Display=エンドユーザ向け (注: 構造体にはDisplayトレイトが導出されていない)
    // println!("rect1 is {:?}", rect1); // {}: Debug=デバッグ向け
    // println!("rect1 is {:#?}", rect1);
}
