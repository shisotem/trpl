fn main() {
    let mut count = 0;
    'outer: loop {
        println!("outer: {}", count);
        count += 1;

        if count > 2 {
            loop {
                println!("inner: {}", count);
                count *= 2;

                if count > 14 {
                    break 'outer;
                }
            }
        }
    }
    println!("final count: {}", count);

    let arr = [1, 2, 3, 4, 5];
    for element in arr {
        println!("{}", element);
    }

    // 高速かつ、indexの終端を超えたアクセスによるpanicを起こさないため、whileよりもforが好まれる
    for num in (1..4).rev() {
        println!("{}", num);
    }
}
