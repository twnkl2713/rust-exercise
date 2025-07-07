// topic: basic match expressions

// program requirements:
// * display "one", "two", "three", or "other" based on whether the value of a variable is 1, 2, 3, or some other number rsp

// Notes: 
// * use a variable set to any integer
// * use a match expression to determine which msg to display
// * use an underscrore (_) to match on any value

fn main() {
    // * use a variable set to any integer
    let my_number = 2;

    // * use a match expression to determine which msg to display
    match my_number {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"),
    }
    // * use an underscrore (_) to match on any value

}