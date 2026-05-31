// Topic: External crates
//
// Requirements:
// * Display the current date and time
//
// Notes:
// * Use the `chrono` crate to work with time
// * (OPTIONAL) Read the documentation section `Formatting and Parsing`
//   for examples on how to create custom time formats

// * Use the `chrono` crate to work with time
use chrono::prelude::*;

fn main() {
    let local: DateTime<Local> = Local::now(); // Get the local time
    // * (OPTIONAL) Read the documentation section `Formatting and Parsing`
    //   for examples on how to create custom time formats
    let formatted = local.format("%a %b %e %_I:%M%P").to_string();
    println!("{:?}", formatted);
}
