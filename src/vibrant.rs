use image::{GenericImage, Pixel, Rgb, Rgba};

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Vibrancy {
    pixels: Vec<Rgb<u8>>,
}

impl Vibrancy {
    pub fn new<P, G>(image: &G) -> Vibrancy
        where P: Sized + Pixel<Subpixel = u8>,
              G: Sized + GenericImage<Pixel = P>
    {
        let important_pixels = image.pixels()
                                    .map(|(_, _, pixel)| pixel.to_rgba())
                                    .filter(filter_boring_pixels)
                                    .map(|px| px.to_rgb())
                                    .take(8)
                                    .collect();

        Vibrancy { pixels: important_pixels }
    }
}

fn filter_boring_pixels(pixel: &Rgba<u8>) -> bool {
    let (r, g, b, a) = (pixel[0], pixel[1], pixel[2], pixel[3]);

    // If pixel is mostly opaque and not white
    const MIN_ALPHA: u8 = 125;
    const MAX_COLOR: u8 = 250;

    (a >= MIN_ALPHA) && !(r > MAX_COLOR && g > MAX_COLOR && b > MAX_COLOR)
}
