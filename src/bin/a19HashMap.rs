// notes: hashmap

// requirements:
// * print the name and number of items in stock for a furniture store
// * if the number of items is 0, print "out of stack" instead of 0
// * the store has:
//  - 5 chairs
//  - 3 beds
//  - 2 tables
//  - 0 couches
// * print the total no. of items in stack

// notes:
// * use a hashmap for the furniture store stock


use std::collections::HashMap;

fn main() {
    let mut stock = HashMap::new();
    stock.insert("Chair", 5);
    stock.insert("Bed", 3);
    stock.insert("Table", 2);
    stock.insert("Couch", 0);

    let mut total_stock = 0;
    for (item, qty) in stock.iter() {
        total_stock = total_stock + qty;
        let stock_count = if *qty == 0 {
            "out of stock".to_owned()
        }
        else {
            format!("{:?}", qty)
        };
        println!("item={:?}, stock={:?}", item, stock_count);
    }
    println!("Total stock = {:?}", total_stock);
}