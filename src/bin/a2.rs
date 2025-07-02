// topic: basic arithmetic

// program requirements:
// * displays the result of the sumof two numbers

// notes:
// * use a fn to add two no.s together
// * use a fn to display the result
// * use the "{:?}" token in the println macro to display the result

// * use a fn to add 2 no.s together
fn sum(a: i32, b: i32) -> i32 {
    a + b
}

// * use a fn to add two no.s together
fn display_result(result: i32) {
    // * use the "{:?}" token in the println macro to display the result
    println!("{:?}", result);
}

fn main() {
    let result = sum(2, 2);
    display_result(result);
}