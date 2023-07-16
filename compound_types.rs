// COMPOUND TYPES:
//
// ARRAYS are used to store multiple values OF THE SAME TYPE like [1, 2, 3]

fn main() {
    // let variable_name = [value; length];
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    for number in numbers.iter() {
        println!("{}", number);
    }
}

// TUPLES are used to store muliple values of same or different types like (1, true)
fn main() {
    let person: (&str, i32, bool) = ("Alice", 30, true);

    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Is married?: {}", person.2);
}