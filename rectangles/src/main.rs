#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

fn main() {
    let width = 30;
    let height = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height)
    );

    let dimensions = (width, height);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(dimensions)
    );

    let rect = Rectangle { width, height };
    println!("Display the rectangle:\n{rect:?}");
    println!("Display the rectangle:\n{rect:#?}");
    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect)
    );
    let scale = 2;
    let rect_debug = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect_debug);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
    if rect.width() {
        println!("The rectangle has a non-zero width.");
    } else {
        println!("The rectangle has a zero width.");
    }

    let mut square = Rectangle::square(3);
    println!("Display the square:\n{square:?}");
    square.set_width(10);
    println!("Display the square after setting width:\n{square:?}");
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectanle: &Rectangle) -> u32 {
    rectanle.width * rectanle.height
}

impl Rectangle {
    fn max(self, other: Self) -> Self {
        let w = self.width.max(other.width);
        let h = self.height.max(other.height);
        Rectangle {
            width: w,
            height: h,
        }
    }

    fn set_to_max(&mut self, other: Rectangle) {
        *self = self.max(other);
    }
}
