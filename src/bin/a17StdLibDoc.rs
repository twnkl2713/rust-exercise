// topic: browsing standard library documentation

// requirements:
// * print a string in lowercase and uppercase

// notes:
// * utilize std lib functionality to transform the string to lowercase and uppercase
// * use 'rustup doc' in a terminal to open the std lib docs
// * navigate to the API documentation section
// * search for functionality to transform a string (or str) to uppercase and lowercase
// * try searching for to_uppercase, to_lowercase

fn main() {
    let my_str = "this is my STRING";
    println!("uppercase: {:?}", my_str.to_uppercase());
    println!("lowercase: {:?}", my_str.to_lowercase());
}