// topic: advanced match

// program requirements:
// * print out a list of tickets and their functionalities for an event
// * tickets can be backstage, vip, and standard
// * backstage and vip tickets include the ticket holder's name
// * all tickets include the price

// notes:
// * use an enum for the tickets with associated with each variant
// * create one of each ticket and place into a vector
// * use a match expression while iterating the vector to print the ticket info

// * backstage and vip tickets include the ticket holder's name
// * all tickets include the price
enum Ticket {
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String),
}

fn main() {
    // * create one of each ticket and place into a vector
    let tickets = vec![
        Ticket::Backstage(50.0, "Billy".to_owned()),
        Ticket::Standard(15.0),
        Ticket::Vip(30.0, "Elsa".to_owned()),
    ];

    // * use a match expression while iterating the vector to print the ticket info
    for ticket in tickets {
        match ticket {
            Ticket::Backstage(price, holder) => println!("Backstage ticket - Holder: {:?}, Price: {:?}", holder, price),
            Ticket::Standard(price) => println!("Standard ticket - Price: {:?}", price),
            Ticket::Vip(price, holder) => println!("VIP ticket - Holder: {:?}, Price: {:?}", holder, price),
        }
    }
}
