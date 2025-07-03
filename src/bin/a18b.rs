// topic: result & the question mark operator 

// summary:
// this small program simulates unlocking a door using digital keycards backed by a database. many rrors can occur when working with a database, making the question mark operatir the perfect thing to use to keep the code manageable

// requirements:
// * write the body of the 'authorize' function. the steps to authorize a user are:
//    1. Connect to the database
//    2. Find the employee with the 'find_employee' database function
//    3. Get a keycard with the 'get_keycard' database function
//    4. Determine if the keycard's 'access_level' is sufficient, 
//         using the 'required_access_level' function implemented on 'ProtectedLocation'.
//         * Higher 'access_level' values grant more access to 'ProtectedLocation'.
//         1000 can access 1000 and lower. 800 can access 500 but not 1000,..
// * Run the program after writing your 'authorize' function. Expected output:
//         Ok(Allow)
//         Ok(Deny)
//         Err("Catherine doesn't have a keycard")
// * Use the question mark operator within the 'authorize' function.

// notes: 
// * only the 'authorize' func should be changed. everything else can remain unmodified

#[derive(Debug, Clone, Copy)]
enum ProtectedLocation {
    All,
    Office,
    Warehouse,
}

impl ProtectedLocation {
    fn required_access_level(&self) -> u16 {
        match self {
            Self::All => 1000,
            Self::Office => 800,
            Self::Warehouse => 500,
        }
    }
}

#[derive(Debug)]
struct Database;

impl Database {
    fn connect() -> Result<Self, String> {
        // in a production application, a database connection error is likely to occur 
        Ok(Database)
    }

    fn find_employee(&self, name: &str) -> Result<Employee, String> {
        match name {
            "Twinkle" => Ok(Employee {
                name: "Twinkle".to_string(),
            }),
            "Saumya" => Ok(Employee {
                name: "Saumya".to_string(),
            }),
            "Soham" => Ok(Employee {
                name: "Soham".to_string(),
            }),
            _ => Err(String::from("employee not found")),
        }
    }

    fn get_keycard(&self, employee: &Employee) -> Result<KeyCard, String> {
        match employee.name.as_str() {
            "Twinkle" => Ok(KeyCard { access_level: 1000 }),
            "Saumya" => Ok(KeyCard { access_level: 500 }),
            other => Err(format!("{other} doesn't have a keyword")),
        }
    }
}

#[derive(Debug, Clone)]
struct Employee {
    name: String,
}

#[derive(Debug)]
struct KeyCard {
    access_level: u16,
}

#[derive(Debug, Clone, Copy)]
enum AuthorizationStatus {
    Allow,
    Deny,
}

fn authorize(
    employee_name: &str,
    location: ProtectedLocation,
) -> Result<AuthorizationStatus, String> {
    // * write the body of the 'authorize' function. the steps to authorize a user are:
    //    1. Connect to the database
    let db = Database::connect()?;
    //    2. Find the employee with the 'find_employee' database function
    let employee = db.find_employee(employee_name)?;
    //    3. Get a keycard with the 'get_keycard' database function
    let keycard = db.get_keycard(&employee)?;
    //    4. Determine if the keycard's 'access_level' is sufficient, 
    //         using the 'required_access_level' function implemented on 'ProtectedLocation'.
    //         * Higher 'access_level' values grant more access to 'ProtectedLocation'.
    //         1000 can access 1000 and lower. 800 can access 500 but not 1000,..
    if keycard.access_level >= location.required_access_level() {
        Ok(AuthorizationStatus::Allow)
    }
    else {
        Ok(AuthorizationStatus::Deny)
    }
}

fn main() {
    println!("{:?}", authorize("Twinkle", ProtectedLocation::All));      
    println!("{:?}", authorize("Saumya", ProtectedLocation::Office));   
    println!("{:?}", authorize("Soham", ProtectedLocation::Office)); 
}
