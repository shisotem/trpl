// Rustの各値は、所有者と呼ばれる変数と対応している。
// いかなる時も所有者は一つである。
// 所有者がスコープから外れたら、値は破棄される。

fn main() {
    // move
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}, world!", s1); // err

    let s = String::from("hello");
    takes_ownership(s); // move
    let x = 5;
    makes_copy(x); // copy

    // println!("{}", s); // err
    println!("{}", x); // ok

    let a = String::from("hello");
    let (b, len) = calculate_length(a);
    println!("The length of '{}' is {}.", b, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // drop

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn calculate_length(s: String) -> (String, usize) {
    // s: move
    let length = s.len();
    (s, length)
} // s, length: move

// 所有権を取り、またその所有権を戻す、ということを全ての関数でしていたら、ちょっとめんどくさいですね。
// 関数に値は使わせるものの所有権を取らないようにさせるにはどうするべきでしょうか。
// => 参照
