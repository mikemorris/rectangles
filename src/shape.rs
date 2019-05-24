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
