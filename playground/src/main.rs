mod food;

use std::fmt::Display;
use food::Cake;
use food::Pizza;
use food::Smoothie;

mod water;
mod trait_objects;
mod box_basics;

use water::Coca;
use crate::trait_objects::{Area, Rectangle, Square};

fn show_me1<T: Display>(val: T) {
    println!("{}", val);
}

fn show_me2(val: impl Display) {
    println!("{}", val);
}

fn foo<'a, 'b>(a: &'a str, b: &'b str) -> &'b str {
    b
}

fn main() {
    let _eatable = Cake;
    let _eatable = Smoothie;
    let _eatable = Pizza;
    let _drink = Coca;

    show_me1("trait1");
    show_me2("trait2");

    let s = Square(3f32);
    let r = Rectangle(4f32, 2f32);

    let shapes: Vec<&dyn Area> = vec![&s, &r];
    for s in shapes {
        println!("{:?} area is {}", s, s.get_area());
    }

}
