fn main() {
    // ifは式 (if式)
    let condition = true;
    let number = if condition { 5 } else { 6 }; // { 5 }や{ 6 }をアームという
    println!("The value of number is: {}", number);
}
