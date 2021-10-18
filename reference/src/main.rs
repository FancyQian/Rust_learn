fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s1 = String::from("hello");

    change(&mut s1);
    println!("Changed string is {}.", s1);


    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        r1.push_str(", world1");
        println!("{}", r1);
    }
    let r2 = &mut s;
    r2.push_str(", world2");
    println!("{}", r2);


    println!("========================================");

    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用

    let r3 = &mut s; // 没问题
    println!("{}", r3);

    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> String {
    let s = String::from("reference_to_nothing");

    s
}