fn main() {
    let name = "Alice";
    println!("Hello, {}!", name);  // Prints: Hello, Alice!
    //
    println!("{}", 42);  // Prints: 42
    //
    let pi = 3.14159;
    println!("{:.2}", pi);  // Prints: 3.14
    //
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    println!("The point is at ({}, {})", p.x, p.y);
    //-----
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn square(size: u32) -> Rectangle {
            Rectangle { width: size, height: size }
        }
    }

    // Here, Rectangle::square(3) creates a new Rectangle instance where width and height are both set to 3.
    // However, if we want to create a method that processes the current data structure, we will declare an associated method with self or &self. This is useful if we want to chain and further process the data. For example, getting the area of our Rectangle
    impl Rectangle {
    // other code here
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }
    let sq = Rectangle::square(3);
    let area_sq = sq.area();

    // Enums, short for enumerations, are a type in Rust that allow you to define a type by enumerating its possible variants. Enums are incredibly flexible and can represent data structures as simple as a list of variants or as complex as complex as nested structures.
    enum Direction {
        North,
        South,
        East,
        West,
    }

    let circle = Shape::Circle(10.0);
    let rectangle = Shape::Rectangle(10.0, 20.0);

    println!("Area of circle: {}", area(circle));
    println!("Area of rectangle: {}", area(rectangle));
}


// Enums can be used with match expressions to handle various cases represented by the enum variants. This provides a safe way to deal with enumerations and ensure that all possible cases are handled.
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}

fn area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => 3.14 * radius * radius,
        Shape::Rectangle(width, height) => width * height,
    }
}

