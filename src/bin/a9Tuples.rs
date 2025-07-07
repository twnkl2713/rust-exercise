// topic: tuples

// program requirements:
// * print whether the y-value of a cartesian coordinate is greater than 5, lesser than 5, or equal to 5

// notes:
// * use a fn that returns a tuple
// * destructive the return value into two variables
// * use an if..else if..else block to determine what to print

// * use a fn that returns a tuple
#![allow(dead_code)]
fn coordinate() -> (i32, i32) {
    (1, 7)
}

fn main() {
    // * destructure the return value into two variables
    let (x, y) = coordinate();

    // * use an if..else if..else block to determine what to print
    if y > 5 {
        println!(">5");
    } else if y < 5 {
        println!("<5");
    } else {
        println!("=5");
    }
}