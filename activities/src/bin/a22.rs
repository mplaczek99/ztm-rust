// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    }
    Some(a / b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}

// * Write tests for the existing program to ensure proper functionality.
#[cfg(test)]
mod test {
    use crate::*;

    // * Create at least two test cases for each function.
    #[test]
    fn clamp_returns_number_when_inside_range() {
        let result = clamp(5, 1, 10);
        let expected = 5;

        assert_eq!(result, expected);
    }

    #[test]
    fn clamp_returns_lower_when_number_is_too_low() {
        let result = clamp(-5, 1, 10);
        let expected = 1;

        assert_eq!(result, expected);
    }

    #[test]
    fn clamp_returns_upper_when_number_is_too_high() {
        let result = clamp(50, 1, 10);
        let expected = 10;

        assert_eq!(result, expected);
    }

    #[test]
    fn div_returns_some_when_division_is_valid() {
        let result = div(10, 2);
        let expected = Some(5);

        assert_eq!(result, expected);
    }

    #[test]
    fn div_returns_none_when_dividing_by_zero() {
        let result = div(10, 0);
        let expected = None;

        assert_eq!(result, expected);
    }

    #[test]
    fn concat_combines_two_words_without_space() {
        let result = concat("hello", "world");
        let expected = String::from("helloworld");

        assert_eq!(result, expected);
    }

    // * Use `cargo test` to test the program.
    // * There are intentional bugs in the program that need to be fixed.
    //   * Check the documentation comments for the functions to
    //     determine how the they should operate.
}
