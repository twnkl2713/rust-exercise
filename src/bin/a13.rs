// topic: vectors

// requirements: 
// * print 10, 20, "thirty", and 40 in a loop
// * print the total no. of elements in a vector

// notes:
// * use a vector to store 4 numbers
// * iterate through the vector using a for..in loop
// * determine whether to print the number or print "thirty" inside the loop
// * use the .len() fun to print the no of elements in a vector

fn main() {
    // * use a vector to store 4 numbers
    let my_numbers = vec![10, 20, 30, 40];
    // * iterate through the vector using a for..in loop
    for num in &my_numbers {
        // * determine whether to print the number or print "thirty" inside the loop
        match num {
            30 => println!("thirty"),
            _ => println!("{:?}", num),
        }
    }
    // * use the .len() fun to print the no of elements in a vector
    println!("number of elements = {:?}", my_numbers.len());
}