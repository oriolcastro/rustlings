// functions1.rs
// Execute `rustlings hint functions1` or use the `hint` watch subcommand for a hint.

/**
 * The function needs to be declared.
 * In this example suggested by codeium we iterate through the numbers 1 to 4.
 * 1..4 creates an iterator but only 1 to 3 are logged into the console
 */

fn call_me() {
    for i in 1..4 {
        println!("Ring! Call number {}", i);
    }
}

fn main() {
    call_me();
}
