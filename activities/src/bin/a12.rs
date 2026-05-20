// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

// * Use an enum for the box color
enum Color {
    Blue,
    Red,
}

impl Color {
    fn print(&self) {
        print!("Color: ");
        match self {
            Color::Blue => println!("Blue"),
            Color::Red => println!("Red"),
        };
    }
}

struct Dimensions {
    length: f64,
    width: f64,
    height: f64,
}

impl Dimensions {
    fn print(&self) {
        println!(
            "Dimensions: {:?} x {:?} x {:?}",
            self.length, self.width, self.height
        );
    }
}

// * Use a struct to encapsulate the box characteristics
struct ShippingBox {
    dimensions: Dimensions,
    weight: f64,
    color: Color,
}

impl ShippingBox {
    // * Implement functionality on the box struct to create a new box
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }

    // * Implement functionality on the box struct to print the characteristics
    fn print_characteristics(&self) {
        println!("Weight: {:?}", self.weight);
        self.color.print();
        self.dimensions.print();
    }
}

fn main() {
    let small_size = Dimensions {
        length: 4.0,
        width: 6.0,
        height: 2.0,
    };
    let my_box = ShippingBox::new(8.0, Color::Blue, small_size);
    my_box.print_characteristics();
}
