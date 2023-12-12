use std::env;
use std::path::Path;
use image::io::Reader;

fn main() {
    let image_path = env::args().skip(1).next().unwrap();
    println!("{}", image_path);
    let path = Path::new(&image_path);
    let img = Reader::open(path).unwrap().decode();
    let rotated = img.unwrap().rotate90();
    rotated.save(path).expect("fail");
}
