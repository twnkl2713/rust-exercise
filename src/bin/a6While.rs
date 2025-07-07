// topic: looping using the while statement

// programm requirements:
// * counts down from 5 to 1, displays the countdown in the terminal, then prints "done!" when complete

// notes:
// * use a mutable integer variable
// * use a while statment
// * print the variable within the while loop
// * do not use the break to exit the loop

fn main() {
    // * use a mutable integer variable
    let mut counter = 5;
    // * use a while statement
    while counter >= 1 {
        // * print the variable within the while loop
        println!("{:?}", counter);
        counter = counter - 1;
    }
    println!("Done!");
}