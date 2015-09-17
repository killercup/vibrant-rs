use std::fmt;
use std::collections::BTreeMap;

use itertools::Itertools;
use image::{GenericImage, Pixel, Rgb, Rgba};
use color_quant::NeuQuant;

use hsl::HSL;
use settings;

#[derive(Debug, Hash, PartialEq, Eq, Default)]
pub struct Vibrancy {
    primary: Option<Rgb<u8>>,
    dark: Option<Rgb<u8>>,
    light: Option<Rgb<u8>>,
    muted: Option<Rgb<u8>>,
    dark_muted: Option<Rgb<u8>>,
    light_muted: Option<Rgb<u8>>,
}

impl Vibrancy {
    pub fn new<P, G>(image: &G) -> Vibrancy
        where P: Sized + Pixel<Subpixel = u8>,
              G: Sized + GenericImage<Pixel = P>
    {
        let mut pixels: Vec<u8> = vec![];
        for (_, _, pixel) in image.pixels() {
            let rgba = pixel.to_rgba();
            if is_boring_pixel(&rgba) {
                continue;
            }

            for subpixel in rgba.channels() {
                pixels.push(*subpixel);
            }
        }

        const QUALITY: i32 = 10; // in [1...30] where 1 is best
        const COLOR_COUNT: usize = 256;

        let quant = NeuQuant::new(QUALITY, COLOR_COUNT, &pixels);

        let pixel_counts = image.pixels()
                                .map(|(_, _, pixel)| quant.index_of(&pixel.to_rgba().channels()))
                                .fold(BTreeMap::new(),
                                      |mut acc, pixel| {
                                          *acc.entry(pixel).or_insert(0) += 1;
                                          acc
                                      });

        let palette: Vec<Rgb<u8>> =
            quant.color_map_rgba()
                 .iter()
                 .chunks_lazy(4)
                 .into_iter()
                 .map(|rgba_iter| {
                         let rgba_slice: Vec<u8> = rgba_iter.cloned().collect();
                         Rgba::from_slice(&rgba_slice).clone().to_rgb()
                     })
                 .unique()
                 .collect();

        // println!("palette: {:?}", palette);
        // println!("pixel_counts: {:?}", pixel_counts);

        generate_varation_colors(&palette, &pixel_counts)
    }

    fn color_already_set(&self, color: &Rgb<u8>) -> bool {
        let color = Some(*color);
        self.primary == color || self.dark == color || self.light == color ||
        self.muted == color || self.dark_muted == color || self.light_muted == color
    }

    fn find_color_variation(&self,
                            palette: &[Rgb<u8>],
                            pixel_counts: &BTreeMap<usize, usize>,
                            luma: &MTM<f64>,
                            saturation: &MTM<f64>)
                            -> Option<Rgb<u8>> {
        let mut max = None;
        let mut max_value = 0_f64;

        let complete_population = pixel_counts.values().fold(0, |acc, c| acc + c);

        for (index, swatch) in palette.iter().enumerate() {
            let s = HSL::from_pixel(swatch).s;
            let l = HSL::from_pixel(swatch).l;

            if s >= saturation.min && s <= saturation.max && l >= luma.min && l <= luma.max &&
               !self.color_already_set(swatch) {
                let population = *pixel_counts.get(&index).unwrap_or(&0) as f64;
                if population == 0_f64 {
                    continue;
                }
                let value = create_comparison_value(s,
                                                    saturation.target,
                                                    l,
                                                    luma.target,
                                                    population,
                                                    complete_population as f64);
                if max.is_none() || value > max_value {
                    max = Some(swatch.clone());
                    max_value = value;
                }
            }
        }

        max
    }

    // fn fill_empty_swatches(self) {
    //     if self.primary.is_none() {
    //         // If we do not have a vibrant color...
    //         if let Some(dark) = self.dark {
    //             // ...but we do have a dark vibrant, generate the value by modifying the luma
    //             let hsl = HSL::from_pixel(&dark).clone()
    //             hsl.l = settings::TARGET_NORMAL_LUMA;
    //         }
    //     }
    // }
}

impl fmt::Display for Vibrancy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Vibrancy {{\n"));

        macro_rules! display_color {
            ($formatter:expr, $name:expr, $color:expr) => {
                {
                    try!(write!($formatter, "\t"));
                    try!(write!($formatter, $name));
                    if let Some(c) = $color {
                        let rgb = c.channels();
                        try!(write!($formatter,
                            " Color: #{:02X}{:02X}{:02X}\n",
                            rgb[0], rgb[1], rgb[2]
                        ));
                    } else {
                        try!(write!($formatter, " Color: None\n"));
                    }
                }
            };
        }

        display_color!(f, "Primary Vibrant", self.primary);
        display_color!(f, "Dark Vibrant", self.dark);
        display_color!(f, "Light Vibrant", self.light);
        display_color!(f, "Muted", self.muted);
        display_color!(f, "Dark Muted", self.dark_muted);
        display_color!(f, "Light Muted", self.light_muted);

        write!(f, "}}")
    }
}

fn generate_varation_colors(palette: &[Rgb<u8>],
                            pixel_counts: &BTreeMap<usize, usize>)
                            -> Vibrancy {
    let mut vibrancy = Vibrancy::default();
    vibrancy.primary = vibrancy.find_color_variation(palette,
                                                     pixel_counts,
                                                     &MTM {
            min: settings::MIN_NORMAL_LUMA,
            target: settings::TARGET_NORMAL_LUMA,
            max: settings::MAX_NORMAL_LUMA,
        },
                                                     &MTM {
            min: settings::MIN_VIBRANT_SATURATION,
            target: settings::TARGET_VIBRANT_SATURATION,
            max: 1_f64,
        });

    vibrancy.light = vibrancy.find_color_variation(palette,
                                                   pixel_counts,
                                                   &MTM {
            min: settings::MIN_LIGHT_LUMA,
            target: settings::TARGET_LIGHT_LUMA,
            max: 1_f64,
        },
                                                   &MTM {
            min: settings::MIN_VIBRANT_SATURATION,
            target: settings::TARGET_VIBRANT_SATURATION,
            max: 1_f64,
        });

    vibrancy.dark = vibrancy.find_color_variation(palette,
                                                  pixel_counts,
                                                  &MTM {
            min: 0_f64,
            target: settings::TARGET_DARK_LUMA,
            max: settings::MAX_DARK_LUMA,
        },
                                                  &MTM {
            min: settings::MIN_VIBRANT_SATURATION,
            target: settings::TARGET_VIBRANT_SATURATION,
            max: 1_f64,
        });

    vibrancy.muted = vibrancy.find_color_variation(palette,
                                                   pixel_counts,
                                                   &MTM {
            min: settings::MIN_NORMAL_LUMA,
            target: settings::TARGET_NORMAL_LUMA,
            max: settings::MAX_NORMAL_LUMA,
        },
                                                   &MTM {
            min: 0_f64,
            target: settings::TARGET_MUTED_SATURATION,
            max: settings::MAX_MUTED_SATURATION,
        });

    vibrancy.light_muted = vibrancy.find_color_variation(palette,
                                                         pixel_counts,
                                                         &MTM {
            min: settings::MIN_LIGHT_LUMA,
            target: settings::TARGET_LIGHT_LUMA,
            max: 1_f64,
        },
                                                         &MTM {
            min: 0_f64,
            target: settings::TARGET_MUTED_SATURATION,
            max: settings::MAX_MUTED_SATURATION,
        });

    vibrancy.dark_muted = vibrancy.find_color_variation(palette,
                                                        pixel_counts,
                                                        &MTM {
            min: 0_f64,
            target: settings::TARGET_DARK_LUMA,
            max: settings::MAX_DARK_LUMA,
        },
                                                        &MTM {
            min: 0_f64,
            target: settings::TARGET_MUTED_SATURATION,
            max: settings::MAX_MUTED_SATURATION,
        });

    vibrancy
}

fn is_boring_pixel(pixel: &Rgba<u8>) -> bool {
    let (r, g, b, a) = (pixel[0], pixel[1], pixel[2], pixel[3]);

    // If pixel is mostly opaque and not white
    const MIN_ALPHA: u8 = 125;
    const MAX_COLOR: u8 = 250;

    let interesting = (a >= MIN_ALPHA) && !(r > MAX_COLOR && g > MAX_COLOR && b > MAX_COLOR);

    !interesting
}

fn invert_diff(val: f64, target_val: f64) -> f64 {
    1_f64 - (val - target_val).abs()
}

fn weighted_mean(vals: &[(f64, f64)]) -> f64 {
    let (sum, sum_weight) = vals.iter().fold((0_f64, 0_f64),
                                             |(sum, sum_weight), &(val, weight)| {
                                                 (sum + val * weight, sum_weight + weight)
                                             });

    sum / sum_weight
}

fn create_comparison_value(sat: f64,
                           target_sat: f64,
                           luma: f64,
                           target_uma: f64,
                           population: f64,
                           max_population: f64)
                           -> f64 {
    weighted_mean(&[
        (invert_diff(sat, target_sat),   settings::WEIGHT_SATURATION),
        (invert_diff(luma, target_uma),  settings::WEIGHT_LUMA),
        (population / max_population,    settings::WEIGHT_POPULATION),
    ])
}

/// Minimum, Maximum, Target
#[derive(Debug, Hash)]
struct MTM<T> {
    min: T,
    target: T,
    max: T,
}
