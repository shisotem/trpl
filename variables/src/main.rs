const MAX_A: u32 = 100_100;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const MAX_B: u32 = 100_100;

    // shadowing
    let y = 3;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }
    println!("The value of y is : {}", y);

    let spaces = "    ";
    let spaces = spaces.len();
}
