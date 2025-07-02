// topic: implementing functionality using impl keyword

// requirements:
// * print the characteristics of a shiping box
// * must include dimensions, weight, and colour

// notes:
// * use a struct to encapsulate the box characteristics
// * use an enum for the box colour
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

// * use an enum for the box colour
enum Color {
    Brown,
    Red
}

impl Color {
    fn print(&self) {
        match self {
            Color::Brown => println!("brown"),
            Color::Red => println!("red"),
        }
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}

// * use a struct to encapsulate the box characteristics
// * must include dimensions, width, and colour
struct ShippingBox {
    color: Color,
    weight: f64,
    dimensions: Dimensions
}

// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
impl ShippingBox {
    fn new(weight: f64, color: Color, dimensions: Dimensions) -> Self {
        Self {
            weight,
            color,
            dimensions,
        }
    }
    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("weight: {:?}", self.weight);
    }
}

fn main() {
    let small_dimensions = Dimensions {
        width: 1.0,
        height: 2.0,
        depth: 3.0,
    };

    let small_box = ShippingBox::new(5.0, Color::Red, small_dimensions);
    small_box.print();
}