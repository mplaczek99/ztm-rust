// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
struct Student {
    name: String,
    // * The locker assignment should use an Option<i32>
    locker: Option<i32>,
}

fn main() {
    let billy = Student {
        name: "Billy".to_owned(),
        locker: None,
    };
    println!("Student: {:?}", billy.name);

    match billy.locker {
        Some(num) => println!("Billy's locker is: {:?}", num),
        None => println!("Billy does not have a locker"),
    }
}
