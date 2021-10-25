#[warn(unused_variables)]

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // panic!("Hello, world!");
    // let v = vec![1, 2, 3, 4];
    // v[99];

    /* Reslut <T, E> */
    let f = File::open("hello.text");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.text") {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating file! {:?}", e),
            },
            other_error => panic!("There is a unknown error {:?}", other_error),
        },
    };
}
