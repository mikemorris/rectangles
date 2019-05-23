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
}
