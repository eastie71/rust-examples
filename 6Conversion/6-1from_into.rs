use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

// Define `From`
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let int = 5;
    let num1 = Number::from(30i32);
    println!("My number1 is {:?}", num1);
    // use `Into`
    let num2: Number = int.into();
    println!("My number2 is {:?}, value = {}", num2, num2.value);
}