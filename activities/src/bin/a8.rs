// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

// * Use a struct to store drink flavor and fluid ounce information
struct Drink {
    flavor: Flavors,
    fluid_ounces: f64,
}

// * Use an enum to create different flavors of drinks
enum Flavors {
    Berry,
    Vanhilla,
}

// * Use a function to print out the drink flavor and ounces
fn print_drink(drink: Drink) {
    // * Use a match expression to print the drink flavor
    match drink.flavor {
        Flavors::Berry => println!("Berry Flavor"),
        Flavors::Vanhilla => println!("Vanhilla Flavor"),
    }
    println!("{:?} oz", drink.fluid_ounces);
}

fn main() {
    let drink = Drink {
        flavor: Flavors::Berry,
        fluid_ounces: 12.0,
    };

    // Print the drink
    print_drink(drink);
}
