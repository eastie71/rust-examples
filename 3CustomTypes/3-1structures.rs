// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    println!("Width: {1} - {0} = {2:.2}", rect.top_left.x, rect.bottom_right.x, rect.bottom_right.x - rect.top_left.x);
    println!("Height: {0} - {1} = {2:.2}", rect.top_left.y, rect.bottom_right.y, rect.top_left.y - rect.bottom_right.y );
    let width: f32 = rect.bottom_right.x - rect.top_left.x;
    let height: f32 = rect.top_left.y - rect.bottom_right.y;
    width * height
}

fn square(top_left: Point, size: f32) -> Rectangle {
    Rectangle {
        top_left: Point {x: top_left.x, y: top_left.y},
        bottom_right: Point {x: top_left.x + size, y: top_left.y - size},
    }
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 5.2, y: 0.4 };
    let another_point: Point = Point { x: 10.3, y: 0.2 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 10.3, ..another_point };

    // `bottom_right.y` will be the same as `another_point.y` because we used that field
    // from `another_point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let area: f32 = rect_area(_rectangle);
    println!("Rect Area = {:.3}", area);

    let tl: Point = Point {x: 10.5, y: 20.5};
    let my_square: Rectangle = square(tl, 7.2);
    let square_area: f32 = rect_area(my_square);
    println!("Square Area = {:.3}", square_area);
}