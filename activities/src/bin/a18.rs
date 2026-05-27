// Topic: Result
//
// Requirements:
// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
// * Implement a `new` function for the `Adult` structure that returns a Result:
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
// * Instantiate two `Adult` structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message

//   * Implement Debug print functionality using `derive`
#[derive(Debug)]
// * Create an structure named `Adult` that represents a person aged 21 or older:
struct Adult {
    //   * The structure must contain the person's name and age
    name: String,
    age: u8,
}

// * Implement a `new` function for the `Adult` structure that returns a Result:
impl Adult {
    fn new(name: &str, age: u8) -> Result<Self, &str> {
        if age >= 21 {
            Ok(Self {
                age,
                name: name.to_owned(),
            })
        } else {
            Err("Age must be 21 or over")
        }
    }
}

fn print_information(result: &Result<Adult, &str>) {
    match result {
        Ok(a) => println!("Created adult: {:?} {:?}", a.name, a.age),
        Err(e) => println!("Could not create adult: {:?}", e),
    }
}

fn main() {
    // * Instantiate two `Adult` structures:
    let adult_1 = Adult::new("Sebastian", 19);
    let adult_2 = Adult::new("Michael", 26);

    print_information(&adult_1);
    print_information(&adult_2);
}
