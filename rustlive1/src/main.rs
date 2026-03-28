// References and Borrowing
// Safety and Performance
// Borrowing and reference anre power concepts

// Understanding References
// References: Enable you to borrow values without taking ownership
// Immutable Reference.
// Mutable Reference.

fn main() {
    let _x: i32 = 5;
    let _r = &_x;

    println!("value of _x : {}", _x);
    println!("value of _r : {}", _r);
}
