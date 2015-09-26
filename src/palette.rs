use std::fmt;
use std::collections::BTreeMap;

use itertools::Itertools;
use image::{GenericImage, Pixel, Rgb, Rgba};
use color_quant::NeuQuant;

/// Palette of colors.
#[derive(Debug, Hash, PartialEq, Eq, Default)]
pub struct Palette {
    /// Palette of Colors represented in RGB
    pub palette: Vec<Rgb<u8>>,
    /// A map of indices in the palette to a count of pixels in approximately that color in the
    pub pixel_counts: BTreeMap<usize, usize>,
}

impl Palette {
    /// Create a new palett from an image
    ///
    /// Color count and quality are given straight to [color_quant], values should be between
    /// 8...512 and 1...30 respectively. (By the way: 10 is a good default quality.)
    ///
    /// [color_quant]: https://github.com/PistonDevelopers/color_quant
    pub fn new<P, G>(image: &G, color_count: usize, quality: i32) -> Palette
        where P: Sized + Pixel<Subpixel = u8>,
              G: Sized + GenericImage<Pixel = P>
    {
        let pixels: Vec<Rgba<u8>> = image.pixels()
                                         .map(|(_, _, pixel)| pixel.to_rgba())
                                         .collect();

        let mut flat_pixels: Vec<u8> = Vec::with_capacity(pixels.len());
        for rgba in &pixels {
            if is_boring_pixel(&rgba) {
                continue;
            }

            for subpixel in rgba.channels() {
                flat_pixels.push(*subpixel);
            }
        }

        let quant = NeuQuant::new(quality, color_count, &flat_pixels);

        let pixel_counts = pixels.iter()
                                 .map(|rgba| quant.index_of(&rgba.channels()))
                                 .fold(BTreeMap::new(),
                                       |mut acc, pixel| {
                                           *acc.entry(pixel).or_insert(0) += 1;
                                           acc
                                       });

        let palette: Vec<Rgb<u8>> = quant.color_map_rgba()
                                         .iter()
                                         .chunks_lazy(4)
                                         .into_iter()
                                         .map(|rgba_iter| {
                                             let rgba_slice: Vec<u8> = rgba_iter.cloned().collect();
                                             Rgba::from_slice(&rgba_slice).clone().to_rgb()
                                         })
                                         .unique()
                                         .collect();

        Palette { palette: palette, pixel_counts: pixel_counts }
    }

    fn frequency_of(&self, color: &Rgb<u8>) -> usize {
        let index = self.palette.iter().position(|x| x.channels() == color.channels());
        if let Some(index) = index {
            *self.pixel_counts.get(&index).unwrap_or(&0)
        } else {
            0
        }
    }

    /// Change ordering of colors in palette to be of frequency using the pixel count.
    pub fn sort_by_frequency(&self) -> Self {
        let mut colors = self.palette.clone();
        colors.sort_by(|a, b| self.frequency_of(&a).cmp(&self.frequency_of(&b)));

        Palette { palette: colors, pixel_counts: self.pixel_counts.clone() }
    }
}

fn is_boring_pixel(pixel: &Rgba<u8>) -> bool {
    let (r, g, b, a) = (pixel[0], pixel[1], pixel[2], pixel[3]);

    // If pixel is mostly opaque and not white
    const MIN_ALPHA: u8 = 125;
    const MAX_COLOR: u8 = 250;

    let interesting = (a >= MIN_ALPHA) && !(r > MAX_COLOR && g > MAX_COLOR && b > MAX_COLOR);

    !interesting
}

impl fmt::Display for Palette {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let color_list = self.palette
                             .iter()
                             .map(|rgb| format!("#{:02X}{:02X}{:02X}", rgb[0], rgb[1], rgb[2]))
                             .join(", ");

        write!(f, "Color Palette {{ {} }}", color_list)
    }
}
