
// For debugging
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        sefl.width > other.width && self.height > other.height
    }

    //associated functions
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area1(width1, height1)
    );

    let rect1 = (30, 50);
    println!(
        "The area of the rectangle in {} square pixels.",
        area2(rect1)
    );

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect2)
    );

    println!("rect2 is {:#?}", rect2);

    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect3);

    let rect4 = Rectangle {
        width: 30,
        height: 50,
    }

    println!(
        "The area of the rectangle is {} square pixels",
        rect4.area()
    )

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    //Calling an associate function
    let sq = Rectangle::square(3);
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// More concise, but less clear, should use structs
fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// Clear, but not sure how these params are related
fn area1(width: u32, height: u32) -> u32 {
    width * height
}
