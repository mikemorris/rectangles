mod shape {
    #[derive(Debug, Default)]
    pub struct Rectangle {
        width: u32,
        height: u32,
        name: Option<String>,
    }

    impl Rectangle {
        pub fn new(width: u32, height: u32) -> Rectangle {
            Rectangle {
                width,
                height,
                ..Default::default()
            }
        }

        pub fn area(&self) -> u32 {
            self.width * self.height
        }

        pub fn set_name(&mut self, name: &str) {
            self.name = Some(name.to_string());
        }

        pub fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
}

fn main() {
    use shape::Rectangle;

    let mut rect = Rectangle::new(30, 50);

    println!("The area of {:#?} is {} pixels.", rect, rect.area());

    rect.set_name("bar");

    println!("{:#?}", rect);

    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::new(10, 40);
    let rect3 = Rectangle::new(60, 45);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
