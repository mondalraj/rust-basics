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
}
