use std::fmt;

struct AGE(i32);

impl fmt::Display for AGE {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "年龄是{}", self.0)
    }
}

struct FloatRoughly(f64);

impl fmt::Display for FloatRoughly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = (self.0 * 100.0) as i32;
        let b = a as f64 / 100.0;
        write!(f, "{}", b)
    }
}

impl fmt::Binary for FloatRoughly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.0 as i32;
        fmt::Binary::fmt(&val, f) // delegate to i32's implementation
    }
}

fn main() {
    println!("Hello, world!");

    println!("{0}, this is {1}. {1}, this is {0}", "Frank", "Qian");

    println!(
        "{name_cn} last name is {name_last}, english name is {name_eng}",
        name_last = "Qian",
        name_eng = "Frank",
        name_cn = "Xiaowei"
    );

    println!(
        "10进制{number}, 2进制{number:b}, 8进制{number:o}, 16进制{number:X}",
        number = 200
    );

    println!("{number:>width$}", number = 1, width = 6);
    println!("{number:>0width$}", number = 1, width = 6);

    let frank_age = AGE(28);

    println!("{}", frank_age);

    let pi = FloatRoughly(3.14159261111111);

    println!("Pi is roughly {}", pi);

    println!("What does Pi look like in binary: {:b}", pi);
}
