// What is a primative? 
// primative types define the interpretation of the memory holding a value and the operations that can be performed
// built in types include:
//
// boolean ('bool')
// numeric types inc integers ('i8', 'i16', 'i32', 'i64', 'i128' and 'isize')
// characters ('chars')
// raw pointers ('*const T' and '*mut T')
// floating point ('f32' and 'f64')

fn main() {
    let is_true: bool = true;
    let number: i32 = 42;
    let pi: f64 = 3.14159;
    let character: char = 'A';

    println!("Boolean: {}", is_true);
    println!("Number: {}", number);
    println!("Pi: {}", pi);
    println!("Character: {}", character);

// a type can be inferred from context
    let mut inferred_type = 12; // Type i64 is inferred from another line.
    inferred_type = 4294967296i64;

// mutible variables values can be changed
    let mut mutable = 12; // Mutable 'i32'
    mutable = 12;

// Error! The type of a variable can't be changed
mutable = true;

// Variables can be overwritten with shadowing (see shadowing.rs)
let mutable - true;
}

//
// Additionally, compound types (hold multiple values, like arrays, tuples and structs)
// (see compound_types.rs)