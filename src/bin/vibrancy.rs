extern crate image;
extern crate vibrant;

use std::env;
use std::path::Path;

use image::GenericImage;
use vibrant::Vibrancy;

fn main() {
    let source = env::args().skip(1).next()
        .expect("No source image given.");
    let img = image::open(&Path::new(&source))
        .ok().expect(&format!("Could not load image {:?}", source));

    println!("{:?}", img.dimensions());

    println!("{:?}", Vibrancy::new(&img));
}
