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
//     // push_strはStringに対して定義されているが、
//     // Rustにはメソッド呼び出し時に必要なだけ自動でデリファレンスを行う仕組みがある
//     // 概念的にはこう: (*some_string).push_str(", world");
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

// // -- cf. slices/src/main.rs
// // <Q.>
// r1はStringではなく、&String型なのに、println!するとhelloと表示されるのはなぜ？
// let &r1 = &s; のように受け取る必要はないの？
// let s = String::from("hello");
// let r1 = &s;
// println!("{}", r1); // hello
// // <A.>
// ごもっともな疑問ですね！`r1`は`&String`型なのに、`println!("{}", r1);`で`hello`と表示されるのは、Rustの**`Display`トレイト**と**型強制（型コージョン）**の仕組みが働いているからです。

// 詳しくご説明します。
// 1.  **`println!`マクロと`Display`トレイト**:
//     `println!("{}", value)`のように`{}`プレースホルダを使うと、Rustは`value`が`std::fmt::Display`トレイトを実装しているかどうかを確認します。`String`型は標準ライブラリで`Display`トレイトを実装しており、その実装では文字列の内容をフォーマットして出力するようになっています。
// 2.  **`&String`型への`Display`トレイトの実装**:
//     重要な点として、`String`型だけでなく、その参照型である`&String`型も`Display`トレイトを実装しています。`&String`に対する`Display`の実装は、参照が指す元の`String`の内容を出力するようになっています。
// 3.  **型強制（型コージョン）**:
//     Rustには、特定の状況下でコンパイラが自動的に型をより一般的な型に変換する「型強制（型コージョン）」という仕組みがあります。`println!`マクロは、引数として様々な型を受け取れるように設計されています。`&String`型は、`println!`マクロが期待する可能性のある型（例えば、`&str`など）に自動的に型強制されることがあります。
// したがって、`println!("{}", r1);`と書いた場合、`r1`（`&String`型）は、`Display`トレイトの実装によって、参照先の`String`の内容である`"hello"`として適切にフォーマットされ、出力されるのです。

// **`let &r1 = &s;` のように受け取る必要がない理由**:
// `let &r1 = &s;`という書き方は、`&s`という`&String`型の参照に対して、さらに参照外し（デリファレンス）を行うパターンマッチングです。この場合、`r1`の型は`String`になります。
// 今回のケースでは、`println!`マクロは`&String`型を直接受け取って処理できるため、わざわざ`String`型として受け取る必要はありません。`r1`が`&String`型のままでも、`Display`トレイトの実装によって期待通りの出力が得られるからです。

// **まとめ**:
// `println!("{}", r1);`で`"hello"`と表示されるのは、`&String`型が`Display`トレイトを実装しており、その実装が参照先の`String`の内容を出力するように定義されているためです。また、必要に応じて型強制も働くことがあります。`let &r1 = &s;`のように書くこともできますが、それは`r1`の型を`String`にしたい場合であり、今回の`println!`の目的においては必須ではありません。
// // --
