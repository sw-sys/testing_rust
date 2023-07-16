// this is a macro
// printls!

// a macro is like a function in python
// this is how to write a simple macro

macro_rules! say_hello {
    () => {
        println!("Hello!")
    };
}

// calling the macro...

fn main() {
    say_hello!()
}