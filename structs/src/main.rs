#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other_rect: Self) -> bool {
        self.height > other_rect.height && self.width > other_rect.width
    }
}

impl Rectangle {
    fn square(dimension: u32) -> Self {
        Self {
            width: dimension,
            height: dimension,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 10,
        height: 24,
    };

    println!("Area of rect1 is {}", rect1.area());

    let rect2 = Rectangle {
        width: 5,
        height: 15,
    };

    let rect3 = Rectangle {
        width: 25,
        height: 15,
    };

    println!("rect1 can hold rect2 {}", rect1.can_hold(rect2));
    println!("rect1 can hold rect3 {}", rect1.can_hold(rect3));

    let square = Rectangle::square(8);

    println!("Area of square is {}", square.area());
}
