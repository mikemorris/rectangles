#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
    name: String,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn rename(&mut self, name: &str) {
        self.name = name.to_string();
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let mut rect = Rectangle {
        width: 30,
        height: 50,
        name: String::from("foo"),
    };

    println!("The area of {:#?} is {} square pixels.", rect, rect.area());

    rect.rename("bar");

    println!("{:#?}", rect);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
        name: String::from("a"),
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
        name: String::from("b"),
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
        name: String::from("c"),
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
