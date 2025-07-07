// topic: flow control usinf if..else

// program requirements:
// * displays a msg based on the value of a boolean variable
// * when the variable is set to true, display "hello"
// * when the variable is set to false, display "goodbye"

// notes:
// * use a variable set to either true or false
// * use an if..ekse bloc to determine which message to display
// * use the println macro to display messages to the terminal

fn main() {
    // * displays a msg based on the value of a boolean variable
    let my_bool = true;
    // * when the variable is set to true, display "hello"
    if my_bool == true {
        println!("hello");
    }
    else {
        println!("goodbye");
    }
}