use std::cmp;
use image::Pixel;

pub struct HSL {
    pub h: f64, // 0-360 Degree
    pub s: f64, // 0-1 (Percent)
    pub l: f64, // 0-1 (Percent)
}

impl HSL {
    /// Convert Pixel value to HSL
    ///
    /// Algorithm from [go-color] by Brandon Thomson <bt@brandonthomson.com>.
    ///
    /// [go-color]: https://github.com/bthomson/go-color
    pub fn from_pixel<P>(pixel: &P) -> HSL
        where P: Pixel<Subpixel = u8>
    {
        let mut h: f64;
        let s: f64;
        let l: f64;

        let pixel = pixel.clone();
        let rgb = pixel.to_rgb();
        let rgb = rgb.channels();
        let (r, g, b) = (rgb[0], rgb[1], rgb[2]);

        let max = cmp::max(cmp::max(r, g), b);
        let min = cmp::min(cmp::min(r, g), b);

        // Normalized RGB: Divide everything by 255 to get percentages of colors.
        let (r, g, b) = (r as f64 / 255_f64, g as f64 / 255_f64, b as f64 / 255_f64);
        let (min, max) = (min as f64 / 255_f64, max as f64 / 255_f64);

        // Luminosity is the average of the max and min rgb color intensities.
        l = (max + min) / 2_f64;

        // Saturation
        let delta: f64 = max - min;
        if delta == 0_f64 {
    		// it's gray
            return HSL { h: 0_f64, s: 0_f64, l: l};
        }

        // it's not gray
        if l < 0.5_f64 {
            s = delta / (max + min);
        } else {
            s = delta / (2_f64 - max - min);
        }

        // Hue
        let r2 = (((max - r) / 6_f64) + (delta / 2_f64)) / delta;
        let g2 = (((max - g) / 6_f64) + (delta / 2_f64)) / delta;
        let b2 = (((max - b) / 6_f64) + (delta / 2_f64)) / delta;

        h = match max {
            x if x == r => b2 - g2,
            x if x == g => (1_f64 / 3_f64) + r2 - b2,
            _ => (2_f64 / 3_f64) + g2 - r2,
        };

        // Fix wraparounds
        if h < 0 as f64 {
            h += 1_f64;
        } else if h > 1 as f64 {
            h -= 1_f64;
        }

        HSL { h: h * 360_f64, s: s, l: l }
    }
}

#[cfg(test)]
mod test {
    macro_rules! test_rgb_to_hsl {
        (($r:expr, $g:expr, $b:expr) <=> ($h:expr, $s:expr, $l:expr)) => {
            {
                // Round gracefully to half a percent
                const EPSILON: f64 = 0.05;
                // Round gracefully to half a degree
                const EPSILON_DEGREE: f64 = 0.5;

                let rgb = Rgb::from_channels($r as u8, $g as u8, $b as u8, 0_u8);
                let hsl = HSL::from_pixel(&rgb);

                assert!(
                    $h - hsl.h <= EPSILON_DEGREE,
                    "Converting {:?} to HSL: H differs too much. Expected {}, got {}.",
                        rgb, $h, hsl.h
                );
                assert!(
                    $s - hsl.s <= EPSILON,
                    "Converting {:?} to HSL: S differs too much. Expected {}, got {}.",
                        rgb, $s, hsl.s
                );
                assert!(
                    $l - hsl.l <= EPSILON,
                    "Converting {:?} to HSL: L differs too much. Expected {}, got {}.",
                        rgb, $l, hsl.l
                );
            }
        };
    }

    use image::{Rgb, Pixel};
    use super::HSL;

    #[test]
    fn rgb_to_hsl() {
        // black
        test_rgb_to_hsl!( (0, 0, 0) <=> (0_f64, 0_f64, 0_f64) );

        // white
        test_rgb_to_hsl!( (255, 255, 255) <=> (0_f64, 0_f64, 1_f64) );

        // http://rgb.to/rgb/18,35,67
        test_rgb_to_hsl!( (18, 35, 67) <=> (219_f64, 0.58_f64, 0.17_f64) );

        // http://rgb.to/hex/93c6cd
        test_rgb_to_hsl!( (147, 198, 205) <=> (187_f64, 0.37_f64, 0.69_f64) );

        // http://rgb.to/hex/bada55
        test_rgb_to_hsl!( (186, 218, 85) <=> (74_f64, 0.64_f64, 0.59_f64) );

        // http://rgb.to/hex/ff0
        test_rgb_to_hsl!( (255, 255, 0) <=> (60_f64, 1_f64, 0.5_f64) );

        // http://rgb.to/rgb/198,250,172
        test_rgb_to_hsl!( (198, 250, 172) <=> (100_f64, 0.89_f64, 0.83_f64) );

        // http://rgb.to/hex/faadc7
        test_rgb_to_hsl!( (250, 173, 199) <=> (340_f64, 0.89_f64, 0.83_f64) );
    }
}
