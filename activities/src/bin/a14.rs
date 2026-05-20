// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

// * Use a struct for a persons age, name, and favorite color
struct Person {
    age: u8,
    // * The color and name should be stored as a String
    name: String,
    color: String,
}

// * The name and colors should be printed using a function
fn print(data: &str) {
    println!("{:?}", data);
}

fn main() {
    // * Create and store at least 3 people in a vector
    let people = vec![
        Person {
            age: 7,
            name: "George".to_owned(),
            color: String::from("green"),
        },
        Person {
            age: 9,
            name: "Anna".to_owned(),
            color: String::from("purple"),
        },
        Person {
            age: 14,
            name: "Katie".to_owned(),
            color: String::from("blue"),
        },
    ];
    // * Iterate through the vector using a for..in loop
    for person in people {
        // * Use an if expression to determine which person's info should be printed
        if person.age < 10 {
            print(&person.name);
            print(&person.color);
        }
    }
}
