// topic: ownership

// program requirements:
// * print out the qty and id no. of a grocery item

// notes:
// * use a struct for the grocery item
// * use two i32 fields for the qty and id no.
// * create a fun to display the qty
// * create a fun to display the id no.

// * use a struct for the grocery item
struct GroceryItem {
    // * use two i32 fields for the qty and id no.
    quantity: i32,
    id: i32
}

// * create a fun to display the qty
fn display_quantity(item: &GroceryItem) {
    println!("quantity: {:?}", item.quantity);
}

// * create a fun to display the id no.
fn display_id(item: &GroceryItem) {
    println!("id: {:?}", item.id);
}

fn main() {
    let my_item = GroceryItem {
        quantity: 3,
        id: 99,
    };
    display_quantity(&my_item);
    display_id(&my_item);
}
