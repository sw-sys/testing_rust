// run in terminal e.g. rustc 1c_comments.rs
// Cargo.toml set up https://doc.rust-lang.org/cargo/reference/manifest.html


// Regular comments, ignored by the compiler
static MY_NUMBER: i32 = 19; // they're just for one line 

// or to explain a function - add a & b
fn main() {
    println!("My number is {}", MY_NUMBER);
}

/* Block comments which go to the closing delimiter 
these are used to span multiple lines
and are still ignored by the compiler.
They're used to temp disable code blocks */