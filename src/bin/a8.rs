// topic: organizing simila data using structs

// requirements:
// * print the flavor of a drink and it's fluid ounces

// notes:
// * use an enum to create different flavors of drinks
// * use a struct to store drink flavor and fluid ounce information
// * use a fn to print out the drink flavor and ounces
// * use a match expression to print the drink flavor

// * use an enum to create different flavors of drinks
#[allow(dead_code)]
enum Flavor {
    Sparkling,
    Sweet,
    Fruity
}

// * use a struct to store drink flavor and fluid ounce information
struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}

// * use a fn to print out the drink flavor and ounces
fn print_drink(drink: Drink) {
    // * use a match expression to print the drink flavor
    match drink.flavor {
        Flavor::Sparkling => println!("flavor: sparkling"),
        Flavor::Sweet => println!("flavor: sweet"),
        Flavor::Fruity => println!("flavor: fruity"),
    }
    println!("oz: {:?}", drink.fluid_oz);
}
fn main() {
    let sweet = Drink {
        flavor: Flavor::Sweet,
        fluid_oz: 6.0
    };
    print_drink(sweet);
    let fruity = Drink {
        flavor: Flavor::Fruity,
        fluid_oz: 10.0
    };
    print_drink(fruity);
}