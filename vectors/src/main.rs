fn main() {
    // 基本
    let a: Vec<i32> = Vec::new();
    let b = vec![1, 2, 3];
    let mut c = Vec::new();
    c.push(5);
    c.push(6);

    // ベクタの要素を読む2つの方法
    let d = vec![1, 2, 3, 4, 5];

    let third: &i32 = &d[2];
    println!("The third element is {}", third);

    match d.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // 存在しない要素への参照
    // let does_not_exist = &d[100]; // panic
    let does_not_exist = d.get(100); // => None

    // 借用規則 (borrow-checker)
    let mut e = vec![1, 2, 3, 4, 5];
    // let first = &e[0];
    // e.push(6);
    // println!("The first element is: {}", first);

    // for
    let mut f = vec![1, 2, 3, 4, 5];
    for i in &mut f {
        *i += 50;
    }
    println!("{:?}", f);

    // 異なる型の要素をVecに保持したい場合 => enumを使うテクニック
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("red")),
        SpreadsheetCell::Float(10.12),
    ];
}
