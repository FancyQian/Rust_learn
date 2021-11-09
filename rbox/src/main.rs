use std::ops::Deref;
use std::mem::drop;
use std::rc::Rc;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    println!("x address is {:p}, y address is {:p}", &x, &y);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // 强制解引用，因为String实现了Dref
    let hl = String::from("Jay");

    hello(&hl);

    // Drop trait
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };

    // 提前手动drop
    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointers created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");

    // RC: reference counter
    let a = Rc::new(RcList::Cons(5, Rc::new(RcList::Nil)));
    let b = RcList::Cons(3, a.clone());
    let c = RcList::Cons(4, a.clone());
}

fn hello(name: &str) {
    println!("Hello, {}!", name);

}