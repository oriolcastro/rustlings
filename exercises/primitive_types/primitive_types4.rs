// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand for a hint.

/**
 * Uses a range to construct a slice of the original array.
 * The nice_slice is not "linked" to the original array, so if that changes the value will not be updated
 */
#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4];
    // This does not work because a is borrowed in the previous line and is not declared mutable.
    //a = [6, 7, 8, 9, 10];

    assert_eq!([2, 3, 4], nice_slice)
}
