fn main() {
    let mut str1 = String::from("Init");
    let str2 = " me!";
    str1.push_str(str2);
    println!("str2: {} point: {:p}", str2, &str2);
    println!("str1: {} {:p}", str1, &str1);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; 

    // println!("s1 {}", s1);
    println!("s2 {}", s2);
    println!("s3 {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("ss {}", s);
    println!("s1 {}", s1);
    println!("s2 {}", s2);
    println!("s3 {}", s3);
}
