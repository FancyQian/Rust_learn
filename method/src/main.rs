#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    /* 方法 */
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        rect.width < self.width && rect.height < self.height
    }
    /* 关联函数 */
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The the rectangle has width {}",
        rect1.width()
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square1 = Rectangle::square(30);

    println!(
        "The area of the square1 is {} square pixels.",
        square1.area()
    );
}
