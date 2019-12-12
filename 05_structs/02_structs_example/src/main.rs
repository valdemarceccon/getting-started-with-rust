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
        return self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle{ width: 30, height: 50};
    let rect2 = Rectangle{ width: 15, height: 30};
    let rect3 = Rectangle{ width: 50, height: 30};
    let rect4 = Rectangle::square(50);
    
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!(
        "rect1 can hold rect2: {}",
        rect1.can_hold(&rect2)
    );

    println!(
        "rect1 can hold rect3: {}",
        rect1.can_hold(&rect3)
    );

    print!("{:#?}", rect4);
}
