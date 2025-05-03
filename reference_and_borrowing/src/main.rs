fn main() {
    let a = String::from("hello");
    let len = calculate_length(&a);
    println!("The length of '{}' is {}.", a, len); // 参照のお陰で、sを返すように関数を実装せずとも、ここでsが使える
}

fn calculate_length(s: &String) -> usize {
    // sは参照であり、所有者ではない (sへのmoveも起きていない)
    s.len()
}
