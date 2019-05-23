#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
    name: String,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
        name: String::from("foo"),
    };

    println!("The area of {:#?} is {} square pixels.", rect, area(&rect));
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
