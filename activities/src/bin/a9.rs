// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

// * Use a function that returns a tuple
fn coords() -> (i8, i8) {
    return (1, 5);
}

fn main() {
    // * Destructure the return value into two variables
    let (x, y) = coords();

    // * Use an if..else if..else block to determine what to print
    if y > 5 {
        println!("Greater than 5");
    } else if y < 5 {
        println!("Less than 5");
    } else {
        // Must be equal to 5
        println!("Equal to 5");
    }
}
