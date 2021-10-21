#![allow(unused_variables)]

use std::rc::Weak;
#[derive(Debug)]

enum IpAddrKind {
    V4(u8,u8,u8,u8),
    V6(String)
}


enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn move_sq(&self) {
        match self {
            Message::Move{x, y} => println!("{}", x * y),
            _ => println!("NOT FOUND"),
        }
    }
    fn let_if_move_sq(&self) {
        if let Message::Move{x, y} = self {
            println!("sum {}", x + y);
        }
    }
}

fn main() {
    let ip4 = IpAddrKind::V4(127,0,0,0);
    let ip6 = IpAddrKind::V6(String::from(":::::1"));

    println!("ip4: {:#?}\nip6: {:#?}", ip4, ip6);

    let tom = Message::Move{x:22, y:22};
    tom.move_sq();
    tom.let_if_move_sq();
}
