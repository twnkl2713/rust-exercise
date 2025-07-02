// topic: working with an enum

// program requirements:
// * prints the name of a colour to the terminal

// notes:
// * use an enum with colour names as variants
// * use a fn to print the colour name
// * the fn must use the enum as a parameter
// * use a match expression to determine which color name to print

// * use an enum with colour names as variants
#[allow(dead_code)]
enum Color {
    Red,
    Yellow,
    Blue,
}
// * use a fn to print the colour name
// * the fn must use the enum as a parameter
fn print_color(my_color: Color) {
    // * use a match expression to determine which color name to print
    match my_color  {
        Color::Red => println!("red"),
        Color::Yellow => println!("yellow"),
        Color::Blue => println!("blue"),
    }
}
fn main() {
    print_color(Color::Blue);
}