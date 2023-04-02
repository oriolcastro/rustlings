// functions4.rs
// Execute `rustlings hint functions4` or use the `hint` watch subcommand for a hint.

// This store is having a sale where if the price is an even number, you get
// 10 Rustbucks off, but if it's an odd number, it's 3 Rustbucks off.
// (Don't worry about the function bodies themselves, we're only interested
// in the signatures for now. If anything, this is a good way to peek ahead
// to future exercises!)

fn main() {
    let original_price = 50;
    println!("Your sale price is {}", sale_price(original_price));
}

/**
 * We need to type the return type of the function.
 *
 * if a line has no ; at the end will be returned without the need to to return something;
 *
 * The function is changing the value of the original_price variable passed to the function
 */
fn sale_price(price: i32) -> i32 {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

/**
 * The function returns a boolean depending if the result, using % operator, of dividing the passed number by 2 is 0.
 */
fn is_even(num: i32) -> bool {
    num % 2 == 0
}
