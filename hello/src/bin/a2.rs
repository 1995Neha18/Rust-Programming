struct Dimensions {
    width: u32,
    height: u32,
    depth: u32,
}

impl Dimensions {
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}

struct ShippingBox {
    color: Color,
    weight: u32,
    dimensions: Dimensions,
}

enum Color {
    Red,
    Brown,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Red => println!("Red"),
            Color::Brown => println!("Brown"),
        }
    }
}

impl ShippingBox {
    fn new(weight: u32, color: Color, dimensions: Dimensions) -> Self {
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
    let small_dimensions = Dimensions { width: 10, height: 15, depth: 20 };

    let small_box = ShippingBox::new(10, Color::Red, small_dimensions);
    small_box.print();
}
