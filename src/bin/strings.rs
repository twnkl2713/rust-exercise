// 3 types of commonly used strings

fn main() {
    let name = String::from("Dundun Chicken"); // string type
    let string_slice = &name; // has a 'view' into the original string / is a reference
    let string_literal = "DunnyFox"; // literal is also an &str but it points directly to an address in the binary
    println!("{}", name);
    println!("{}", string_slice);
    println!("{}", string_literal);
}