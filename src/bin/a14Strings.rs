// topic: strings

// program requirements:
// * print out the name and favorite colors of people aged 10 and no.

// notes:
// * use a struct for a persons age, name, and favorite color
// * the color and name should be stored as a string
// * create and store at least 3 people in a vector
// * iterate through the vector using a for..in loop
// * use an if expression to determine which person's info should be printed
// * the name and colors should be printed using a function

// * use a struct for a persons age, name, and favorite color
// * the color and name should be stored as a string
struct Person {
    name: String,
    fav_color: String,
    age: i32,
}

fn print(data: &str) {
    println!("{:?}", data);
}

fn main() {
    // * create and store at least 3 people in a vector
    let people = vec![
        Person {
            name: String::from("Twinkle"),
            fav_color: String::from("black"),
            age: 19,
        },
        Person {
            name: String::from("Soham"),
            fav_color: String::from("black"),
            age: 20,
        },
        Person {
            name: String::from("Saumya"),
            fav_color: String::from("blue"),
            age: 7,
        },
    ];
    // * iterate through the vector using a for..in loop
    for person in people {
        // * use an if expression to determine which person's info should be printed
        if person.age <= 10 {
            print(&person.name);
            print(&person.fav_color);
        }
        // * the name and colors should be printed using a function
    }
}