// Examples using structs
// https://doc.rust-lang.org/book/ch05-01-defining-structs.html

#[derive(Debug)]
struct Rectangle1 {
    width: u32,
    height: u32
}

#[derive(Debug)]
struct Rectangle2 {
    width: u32,
    height: u32
}

// define some methods, and a function "square"
impl Rectangle2 {
    // These are "Methods" as they contain "self" parameter
    fn area(&self) -> u32 {
        self.height * self.width
    }
    fn can_hold(&self, other: &Rectangle2) -> bool {
        self.width > other.width && self.height > other.height
    }
    // Considered "Associated" functions, and use the :: syntax to call them
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle1 {
        width: 30,
        height: 50
    };

    println!("The area of rect1 = {}", area(&rect1));
    println!("rect1 = {rect1:?}");
    // dbg function prints file and line number and nicer format
    dbg!(&rect1);

    let rect2 = Rectangle2 {
        width: 80,
        height: 90
    };

    let rect3 = Rectangle2 {
        width: 30,
        height: 20
    };

    println!("The area of rect2 = {}", rect2.area());
    dbg!(&rect2);
    println!("Can rect2 hold rect3?: {}", rect2.can_hold(&rect3));

    let sq1 = Rectangle2::square(70);
    dbg!(&sq1);
    println!("Can rect3 hold sq1?: {}", rect3.can_hold(&sq1));

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    println!("Config max = {config_max:?}");
}

fn area(rect: &Rectangle1) -> u32 {
    rect.width * rect.height
}