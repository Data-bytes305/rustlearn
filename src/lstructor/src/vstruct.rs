// pub struct Color(i32, i32, i32);
// pub struct Point(i32, i32, i32);

pub struct User{
    pub username : String,
    pub email: String,
    pub sign_in_count: u64,
    pub active: bool,
}

pub struct Rectangle {
    pub  width : u32,
    pub  height: u32,
}

impl Rectangle {
   pub fn area(&self) -> u32 {
        self.height * self.width
    }
}
