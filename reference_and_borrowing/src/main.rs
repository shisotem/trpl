// fn main() {
//     let a = String::from("hello");
//     let len = calculate_length(&a);
//     println!("The length of '{}' is {}.", a, len); // 参照のお陰で、sを返すように関数を実装せずとも、ここでsが使える

//     let mut b = String::from("hello");
//     change(&mut b); // 可変な参照を生成する
// }

// fn calculate_length(s: &String) -> usize {
//     // sは参照であり、所有者ではない (sへのmoveも起きていない)
//     s.len()
// }

// // 可変な参照を受け入れる
// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

fn main() {
    // 可変な参照の制約 (データ競合を防ぐための制約) について
    let mut s = String::from("hello");
    // // ok
    let r1 = &s;
    let r2 = &s;
    // hello, hello
    println!("{}, {}", r1, r2);
    // // err
    // let r1 = &mut s;
    // let r2 = &s;
    // println!("{}, {}", r1, r2);
    // // err
    // let r1 = &s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);
    // // err
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    // // dangling pointer => compile err
    // let reference_to_nothing = dangle();
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// } // s: drop => &s: dangling pointer!?
