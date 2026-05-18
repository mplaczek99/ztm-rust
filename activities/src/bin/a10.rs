// Topic: Working with expressions
//
// Requirements:
// * Print "it's big" if a variable is > 100
// * Print "it's small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

// * Use a function to print the messages
fn print_is_greater(is_gt_100: bool) {
    // * Use a match expression to determine which message
    //   to print
    match is_gt_100 {
        true => println!("it's big"),
        false => println!("it's small"),
    }
}

fn main() {
    // * Use a boolean variable set to the result of
    //   an if..else expression to store whether the value
    //   is > 100 or <= 100
    let num = 999;
    let is_gt_100 = num > 100;
    print_is_greater(is_gt_100);
}
