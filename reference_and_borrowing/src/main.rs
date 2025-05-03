fn main() {
    let a = String::from("hello");
    let len = calculate_length(&a);
    println!("The length of '{}' is {}.", a, len); // 参照のお陰で、sを返すように関数を実装せずとも、ここでsが使える

    let mut b = String::from("hello");
    change(&mut b); // 可変な参照を生成する
}

fn calculate_length(s: &String) -> usize {
    // sは参照であり、所有者ではない (sへのmoveも起きていない)
    s.len()
}

// 可変な参照を受け入れる
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
