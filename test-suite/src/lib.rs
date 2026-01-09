pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn isLargerThan(&self, other: &Rectangle) -> bool {
        self.width * self.height > other.width * other.height
    }

    pub fn is_small(&self) -> bool {
        if(self.width < 10) {
            panic!("rect small");
        }
        self.width * self.height < 100
    }
}
