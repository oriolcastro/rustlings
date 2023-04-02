// primitive_types1.rs
// Fill in the rest of the line that has code missing!
// No hints, there's no tricks, just get used to typing these :)

/**
 * The unary operator works similar as in javascript. You can use it to invert the value of a boolean variable
 */
fn main() {
    // Booleans (`bool`)

    let is_morning = false;
    if is_morning {
        println!("Good morning!");
    }

    let is_evening = !is_morning;
    if is_evening {
        println!("Good evening!");
    }
}
