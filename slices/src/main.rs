fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear(); // s: ""とすると、word: 5はもはやバグのもと!!(同期されていない) => 文字列スライスを使う

    // 文字列スライス: Stringの"一部への"参照
    let mut s2 = String::from("hello world");
    let word2 = first_word2(&s2);
    s2.clear(); // 可変参照
    // println!("the first word is: {}", word2); // compile err (word2: &str...不変参照)
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    // item: u8
    for (i, &item) in bytes.iter().enumerate() {
        // u8 == u8
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
// Note:
// bytes.iter().enumerate()は&u8を返している。
// これをパターンマッチング記法で、&itemで受け取ることで
// 変数itemはu8となり、つまり「参照が指す値（u8）をitemに束縛する」という意味。
// 参照型である&u8から、参照を脱参照して、その内部の値であるu8型の値を取り出しているイメージ。
// => 比較 (item == b' ') などを行う際に、都度、脱参照 (*item) を行う必要がなくなり簡潔になる。
// cf. referece_and_borrowing/src/main.rs

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
