// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

// * Use an enum with color names as variants
enum Colors {
    Red,
    Green,
    Blue,
}

// * Use a function to print the color name
// * The function must use the enum as a parameter
fn print_name(col: Colors) {
    // * Use a match expression to determine which color
    //   name to print
    match col {
        Colors::Red => println!("Red"),
        Colors::Green => println!("Green"),
        Colors::Blue => println!("Blue"),
    }
}

fn main() {
    let color = Colors::Red;
    print_name(color);
}
