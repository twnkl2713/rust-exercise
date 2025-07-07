// topic: basic match expressions

// program requirements:
// * display"it's true" or "it's false" based on the value of a boolean variable

// notes: 
// * use a variable set to either true or false
// * use a match expression to deteremine which message to display

fn main() {
    let my_bool = false;
    match my_bool {
        true => println!("it's true"),
        false => println!("it's a false"),
    }
}
