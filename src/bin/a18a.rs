// topic: result

// program requirements:
// * create a structure named 'Adult' that represents a person aged 21 or older:
// * the structure must contain the person's name and age
// * implement debug print functionality using 'derive'
// * implement a 'new' func for the 'Adult' structure that returns a Result:
// * the Ok variant should contain the initialized structure, but only if the person is aged 21v or older
// * the err variant should contain a String (or &str) that explains why the structure could not be created
// * instantiate two 'Adult' structures:
// * one should be aged under 21
// * one should be 21 or over 
// * use 'match' to print out a message for each 'Adult':
// * for the Ok variant, print any msg u want
// * for the Err variant, print out the error msg

// * create a structure named 'Adult' that represents a person aged 21 or older:
struct Adult {
    // * the structure must contain the person's name and age
    age: u8,
    name: String,
}

impl Adult {
    fn new(age: u8, name: &str) -> Result<Self, &str> {
        // * the Ok variant should contain the initialized structure, but only if the person is aged 21 or older
        if age >= 21 {
            Ok(Self {
                age,
                name: name.to_string()
            })
        } else {
            Err("Age must be at least 21")
        }
    }
}

// * implement debug print functionality using 'derive'
// * implement a 'new' func for the 'Adult' structure that returns a Result:
//  - * the Ok variant should contain the initialized structure, but only if the person is aged 21 or older
//  - * the err variant should contain a String (or &str) that explains why the structure could not be created
// * instantiate two 'Adult' structures:
//  - * one should be aged under 21
//  - * one should be 21 or over 
// * use 'match' to print out a message for each 'Adult':
//  - * for the Ok variant, print any msg u want
//  - * for the Err variant, print out the error msg

fn main() {
    // * instantiate two 'Adult' structures:
    //  - * one should be aged under 21
    let child = Adult::new(19, "Twinkle");
    //  - * one should be 21 or over 
    let adult = Adult::new(21, "Saumya");
    // * use 'match' to print out a message for each 'Adult':
    match child {
        //  - * for the Ok variant, print any msg u want
        Ok(child) => println!("{} is {} years old", child.name, child.age),
        //  - * for the Err variant, print out the error msg
        Err(e) => println!("{e}"),
    }
    
    match adult {
        //  - * for the Ok variant, print any msg u want
        Ok(a) => println!("{} is {} years old", a.name, a.age),
        //  - * for the Err variant, print out the error msg
        Err(e) => println!("{e}"),
    }
}