fn main() {
    // outer block
    let shadowed_binding = 1;

    { // go to inner block
        println!("before being shadowed, value is: {}", shadowed_binding);

        let shadowed_binding = "abc";

        println!("shadowed in inner block, value is now: {}", shadowed_binding);

    }

    // outer block again
    println!("value outside inner block is still: {}", shadowed_binding);

    // change outer block value
    let shadowed_binding = 2;
    println!("shadowed in the outer block: {}", shadowed_binding);
}

// OUTPUT
//
// before being shadowed, value is: 1
// shadowed in inner block, value is now: abc
// value outside inner block is still: 1
// shadowed in the outer block: 2
