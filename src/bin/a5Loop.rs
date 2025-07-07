// topic: looping using the loop statement

// program requirements:
// * display "1" through "4" in the terminal

// Notes:
// * use a mutable integer variable
// * use a loop statement
// * print the variable within the loop statement
// * use break to exit the loop

fn main() {
    // * use a mutable integer variable
    let mut n = 1;
    // * use a loop statement
    loop {
        // * print the variable within the loop
        println!("{:?}", n);
        if n == 4 {
            // * use break to exit the loop
            break;
        }
        n = n + 1;
    }
}