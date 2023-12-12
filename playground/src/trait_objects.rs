use std::fmt::Debug;

#[derive(Debug)]
pub struct Square(pub f32);
#[derive(Debug)]
pub struct Rectangle(pub f32, pub f32);

pub trait Area: Debug {
    fn get_area(&self) -> f32;
}

impl Area for Square {
    fn get_area(&self) -> f32 {
        self.0 * self.0
    }
}

impl Area for Rectangle {
    fn get_area(&self) -> f32 {
        self.0 * self.1
    }
}