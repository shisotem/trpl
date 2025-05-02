use std::io;

fn main() {
    // type annotations needed
    let guess: u32 = "42".parse().expect("Not a number!");

    // 1. Scalar types (整数、浮動小数点数、論理値、文字)
    println!("{}", b'A');
    println!("{}", b'B');

    let c: char = '∇';

    // 2. Compound types (タプル、配列)
    let tup: (i32, f64, u8) = (999, 4.3, 2);
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", tup.1);

    let a: [i32; 5] = [1, 2, 3, 4, 5]; // Rustの配列は固定長(スタック領域に確保) <-> ベクター
    let b = [3; 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let jan = months[0];

    // コンパイルは通るが、5を入力すると、実行時エラーで終了(=panic)する ... Rustの安全機構の一例
    index_out_of_bounds_panic();
}

fn index_out_of_bounds_panic() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
