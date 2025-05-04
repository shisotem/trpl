#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // // メソッド
    // self(ただし稀), &self, &mut selfを使い分ける
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
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

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
