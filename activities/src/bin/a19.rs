// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn main() {
    let mut store = HashMap::new();
    store.insert("Chairs".to_owned(), 5);
    store.insert("Beds".to_owned(), 3);
    store.insert("Tables".to_owned(), 2);
    store.insert("Couches".to_owned(), 0);

    let mut total_number = 0;
    for (item, &number) in store.iter() {
        total_number = total_number + number;
        let count = if number == 0 {
            "out of stock".to_owned()
        } else {
            format!("{:?}", number)
        };
        println!("Item: {:?}, Count: {:?}", item, count);
    }
    println!("{:?} items", total_number);
}
