fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear(); // s: ""とすると、word: 5はもはやバグのもと!!(同期されていない) => 文字列スライスを使う
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
