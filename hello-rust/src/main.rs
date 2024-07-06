fn main() {
    println!("Hello, rust!");
    
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    print!("{}",hello);
    print!("{}",world);
    print!("\n");
    
    let v = vec![1, 2, 3];
    let mut iter = v.iter();
    println!("{}", iter.next().unwrap()); // Prints: 1
    println!("{}", iter.next().unwrap()); // Prints: 2
    println!("{}", iter.next().unwrap()); // Prints: 3
    let result = add(10, 20);
    println!("The sum is {}", result);

    let x = 5;

    // block
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    match number {
        0 => println!("zero"),
        1 => println!("one"),
        _ => println!("something else"),  // exhaustive wildcard. any other matches
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b   // No semicolon here as this is an expression whose value we want to return
}


// Blocks in Rust are fundamental elements of the language. They are used throughout the language in various constructs such as functions, conditional statements (if, else if, else), loops (for, while, loop), and match expressions. A block is essentially a group of statements encapsulated between braces {}. The value of the last expression in the block (if there is one and if it doesn't end with a semicolon) is the value that gets returned from the block. Here is an example:
