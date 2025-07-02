// topic: expressions

// program requirements
// * print "its big" if a variable > 100
// * print "its small" if a variable is <= 100

// notes:
// * use a booolean variable set to an expression that determines whether the value is >100 or <=100
// * use a fun to print the messages
// * use a match expression to determine which message to print

// * use a fun to print the messages
fn print_message(gt_100: bool) {
    // * use a match expression to determine which message to print
    match gt_100 {
        true => println!("its big"),
        false => println!("its small"),
    }
}
fn main() {
    // * use a booolean variable set to an expression that determines whether the value is >100 or <=100
    let value = 100;
    let is_gt_100 = value > 100;
    print_message(is_gt_100);
}
