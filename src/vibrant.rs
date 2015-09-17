use itertools::Itertools;
use image::{GenericImage, Pixel, Rgb, Rgba};
use color_quant::NeuQuant;
use hsl::HSL;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Vibrancy {
    primary: Rgb<u8>,
    dark: Rgb<u8>,
    light: Rgb<u8>,
    muted: Rgb<u8>,
    dark_muted: Rgb<u8>,
    light_muted: Rgb<u8>,
}

impl Vibrancy {
    pub fn new<P, G>(image: &G) -> Vibrancy
        where P: Sized + Pixel<Subpixel = u8>,
              G: Sized + GenericImage<Pixel = P>
    {
        let important_pixels: Vec<Rgb<u8>> = image.pixels()
                                                  .map(|(_, _, pixel)| pixel.to_rgba())
                                                  .filter(filter_boring_pixels)
                                                  .map(|px| px.to_rgb())
                                                  .collect();

        let color_count = 64;

        // FIXME: Use iterators.
        // This didn't work because `[u8]` didn't live long enough
        // let pixels = image.pixels()
        // .flat_map(|(_, _, pixel)| pixel.to_rgba().channels().clone().into_iter().cloned())
        // .collect::<Vec<u8>>()

        let mut pixels: Vec<u8> = vec![];
        for pixel in &important_pixels {
            for subpixel in pixel.channels() {
                pixels.push(*subpixel);
            }
        }

        let palette: Vec<Rgb<u8>> =
            NeuQuant::new(10, color_count, &pixels)
                .color_map_rgba()
                .iter()
                .chunks_lazy(4)
                .into_iter()
                .map(|rgba_iter| {
                        let rgba_slice: Vec<u8> = rgba_iter.cloned().collect();
                        Rgba::from_slice(&rgba_slice).clone().to_rgb()
                    })
                .unique()
                .collect();

        Vibrancy::default()
    }

    fn color_already_set(&self, color: &Rgb<u8>) -> bool {
        let color = *color;
        self.primary == color || self.dark == color || self.light == color ||
        self.muted == color || self.dark_muted == color || self.light_muted == color
    }

    fn findColorVariation(&self,
                          palette: &[Rgb<u8>],
                          luma: &MTM<u8>,
                          saturation: &MTM<u8>)
                          -> Option<Rgb<u8>> {
        let mut max = None;
        let mut maxValue = 0;

        for swatch in palette {
            let s = HSL::from_pixel(swatch).s;
            let l = HSL::from_pixel(swatch).l;

            if s >= saturation.min as f64 && s <= saturation.max as f64 &&
               l >= luma.min as f64 && l <= luma.max as f64 &&
               !self.color_already_set(swatch) {
                /*
                value = @createComparisonValue sat, targetSaturation, luma, targetLuma,
                swatch.getPopulation(), @HighestPopulation
                if max is undefined or value > maxValue
                max = swatch
                maxValue = value
                 */
                max = Some(swatch.clone());
            }
        }

        max
    }
}

impl Default for Vibrancy {
    fn default() -> Vibrancy {
        Vibrancy {
            primary: Rgb::from_channels(0, 0, 0, 0),
            dark: Rgb::from_channels(0, 0, 0, 0),
            light: Rgb::from_channels(0, 0, 0, 0),
            muted: Rgb::from_channels(0, 0, 0, 0),
            dark_muted: Rgb::from_channels(0, 0, 0, 0),
            light_muted: Rgb::from_channels(0, 0, 0, 0),
        }
    }
}

fn filter_boring_pixels(pixel: &Rgba<u8>) -> bool {
    let (r, g, b, a) = (pixel[0], pixel[1], pixel[2], pixel[3]);

    // If pixel is mostly opaque and not white
    const MIN_ALPHA: u8 = 125;
    const MAX_COLOR: u8 = 250;

    (a >= MIN_ALPHA) && !(r > MAX_COLOR && g > MAX_COLOR && b > MAX_COLOR)
}

/// Minimum, Maximum, Target
#[derive(Debug, Hash)]
struct MTM<T> {
    min: T,
    target: T,
    max: T,
}
