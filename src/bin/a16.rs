// topic: advanced match

// program requirements:
// * print out the details of a student's locler assignment
// * lockers use numbers and are optional for students

// notes: 
// * use a struct containing the student's name and locker assignment
// * the locker assignment should use an Option<i32>

// * use a struct containing the student's name and locker assignment
struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let mary = Student {
        name: "Mary".to_owned(),
        locker: Some(3),
    };
    println!("student: {:?}", mary.name);
    match mary.locker {
        Some(num) => println!("locker number: {:?}", num),
        None => println!("no locker assignment"),
    }
}