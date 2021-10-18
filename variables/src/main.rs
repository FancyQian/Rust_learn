fn main() {
    /* 变量默认不可变 */
    let x = 5;
    println!("The value of x is: {}", x);
    /* mut表示该变量可变 */
    let mut x = 6;
    println!("The value of x is: {}", x);
    x = 7;
    println!("The value of x is: {}", x);
    /* Shadowing */
    let spaces = "      ";
    let spaces = spaces.len();
    println!("The length of space is: {}", spaces);
}

