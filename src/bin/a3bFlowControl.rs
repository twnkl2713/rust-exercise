// topic: flow control using if..else if..else

// program requirements:
// * display ">5", "<5", or "=5" based on the value of is > 5, is < 5, or == 5, respectively

// Notes: 
// * use a variable set to any integer value
// * use an if..else if..else blocl to determine which message to display
// * use the println macro to display messages to the terminal

fn main() {
    let n = 7;
    if n > 5 {
        println!(">5");
    }
    else if n < 5 {
        println!("<5");
    }
    else {
        println!("=5");
    }
}