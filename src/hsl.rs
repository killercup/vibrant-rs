use std::cmp::max;
use image::Pixel;

pub struct HSL {
    pub h: f64,
    pub s: f64,
    pub l: f64,
}

impl HSL {
    pub fn from_pixel<P>(pixel: &P) -> HSL
        where P: Pixel<Subpixel = u8>
    {
        let pixel = pixel.clone();
        let rgb = pixel.to_rgb();
        let rgb = rgb.channels();
        let (r, g, b) = (rgb[0] as f64, rgb[1] as f64, rgb[2] as f64);

        let max = *rgb.iter().max().unwrap_or(&255) as f64;
        let min = *rgb.iter().min().unwrap_or(&0) as f64;

        let h: f64;
        let l: f64 = (max + min) as f64 / 2.0f64;
        let s: f64;
        if max == min {
            h = 0;
            s = 0;
        } else {
            let d = max - min;
            s = if l > 0.5 {
                d / (2 - max - min)
            } else {
                d / (max + min)
            };

            h = match max {
                x if x == r => (g - b) / d +
                               (if g < b { 6 } else { 0 }),
                x if x == g => (b - r) / d + 2,
                _ => (r - g) / d + 4,
            } / 6;
        }

        HSL { h: h as f64, s: s as f64, l: l }
    }
}

#[cfg(test)]
mod test {
    macro_rules! test_rgb_to_hsl {
        ($rgb:expr => ($h:expr, $s:expr, $l:expr)) => {
            {
                use std::f64::EPSILON;
                let hsl = HSL::from_pixel(&$rgb);
                assert!(
                    $h - hsl.h <= EPSILON,
                    "Converting {:?} to HSL: H differs too much. Expected {}, got {}.",
                    $rgb, $h, hsl.h
                );
                assert!(
                    $s - hsl.s <= EPSILON,
                    "Converting {:?} to HSL: S differs too much. Expected {}, got {}.",
                    $rgb, $s, hsl.s
                );
                assert!(
                    $l - hsl.l <= EPSILON,
                    "Converting {:?} to HSL: L differs too much. Expected {}, got {}.", $rgb, $l, hsl.l
                );
            }
        };
    }

    use image::{Rgb, Pixel};
    use super::HSL;

    #[test]
    fn rgb_to_hsl() {
        test_rgb_to_hsl!(
            Rgb::from_channels(18u8, 35u8, 67u8, 0u8)
            => (219.0f64, 0.58f64, 0.17f64)
        )
    }
}
