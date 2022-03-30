#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 45,
    };

    println!(
        "The area of the rectangle 1 is {} and 2 are {} square pixels.",
        rect1.area(), rect2.area()
    );
}
