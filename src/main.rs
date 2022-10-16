use std::io;

fn main() {
    // let x = 4; By default all variables declared in rust is immutable
    let mut x = 4; // To make a variable mutable we need to use the keyword mut
    println!("x is {}", x); // x is 4

    x = 5;
    println!("x is {}", x); // x is 5

    // x = "Hello"; -> Error because x is of type i32 and we are trying to assign a string to it (Type mismatch) -> Mutability change value but not type

    // Changing value of a variable without mutation
    let y = 10;
    let y = y + 1;
    println!("y is {}", y); // y is 11

    // Shadowing -> Scope of a variable
    {
        let y = 2;
        println!("y is {}", y); // y is 2
    }
    let y = y + 2;
    println!("y is {}", y); // y is 13 not 2 + 2 = 4

    let y = "Hello";
    println!("y is {}", y); // y is Hello

    // Constants -> Constants are always immutable
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS is {}", MAX_POINTS); // MAX_POINTS is 100000

    // const max_points: u32 = 100_000; -> warning: constant `max_points` should have an upper case name

    // Data Types
    // Scalar Types -> Single value
    // Integer Types -> Signed and Unsigned
    // Signed -> i8, i16, i32, i64, i128, isize
    // Unsigned -> u8, u16, u32, u64, u128, usize
    // isize and usize depends on the kind of computer your program is running on: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.
    // Integer Literals -> Decimal, Hex, Octal, Binary, Byte (u8 only), Char

    // Floating-Point Types -> f32, f64
    // Numeric Operations -> Addition, Subtraction, Multiplication, Division, Remainder

    let sum: i32 = 5 + 10; // Addition
    println!("sum is {}", sum); // sum is 15

    // Boolean Type -> bool
    // Character Type -> char
    let letter: char = 'a';
    println!("letter is {}", letter); // letter is a

    // Compound Types -> Multiple values
    // Tuple Type -> Fixed length
    let mut tup: (bool, f64, char) = (true, 6.4, 'r');
    println!("First value of tup is {}", tup.0); // First value of tup is true

    tup.0 = false;
    println!("tup is {:?}", tup); // tup is (true, 6.4, 'r')

    // Array Type -> Fixed length with same data type
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    arr[3] = 10;
    println!("arr is {:?}", arr); // arr is [1, 2, 3, 10, 5]

    // Collecting User Input
    // Prelude -> Rust automatically imports a number of items into every Rust program in the prelude
    // use std::io; -> To use the io library we need to import it
    let mut guess = String::new(); // String::new() is a function that returns a new instance of a String

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); // io::stdin() returns an instance of std::io::Stdin, which is a type that represents a handle to the standard input for your terminal

    println!("You guessed: {}", guess);

    // Type Casting -> Converting a value from one type to another type
    // let x = 255.0f32; // f64(default) -> f32
    let y = 10_i8; // i32(default) -> i8
    let z = 100 as i64; // i32(default) -> i64

    let a = z / (y as i64);
    println!("a is {}", a); // a is 10

    println!("Max value of i32 is {}", i32::MAX);

    // Converting a String to a Number
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    println!("You guessed: {} + 5: {}", guess, guess + 5);

    // Control Flow
    // if Expressions
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Looping Through a Collection with for
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // Looping Through a Range of Numbers with for
    for number in (1..4).rev() {
        println!("{}!", number);
    }

    // Functions
    let sum: i32 = add_numbers(5, 6);
    println!("sum is {}", sum); // sum is 11

    // Statements and Expressions
    // Statements -> Instructions that perform some action and do not return a value
    // Expressions -> Evaluate to a resulting value
    // let y = (let x = 6); -> Error because let is a statement and not an expression

    let x = {
        let y = 3;
        y + 1 // Expression
    };
    println!("x is {}", x); // x is 4

    // Loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result is {}", result); // result is 20

    // Conditional Loops with while
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn add_numbers(x: i32, y: i32) -> i32 {
    // x + y
    // OR
    return x + y;
}
