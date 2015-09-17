#![allow(dead_code)]

pub const TARGET_DARK_LUMA: f64 = 0.26;
pub const MAX_DARK_LUMA: f64 = 0.45;

pub const MIN_LIGHT_LUMA: f64 = 0.55;
pub const TARGET_LIGHT_LUMA: f64 = 0.74;

pub const MIN_NORMAL_LUMA: f64 = 0.3;
pub const TARGET_NORMAL_LUMA: f64 = 0.5;
pub const MAX_NORMAL_LUMA: f64 = 0.7;

pub const TARGET_MUTED_SATURATION: f64 = 0.3;
pub const MAX_MUTED_SATURATION: f64 = 0.4;

pub const TARGET_VIBRANT_SATURATION: f64 = 1.0;
pub const MIN_VIBRANT_SATURATION: f64 = 0.35;

pub const WEIGHT_SATURATION: f64 = 3.0;
pub const WEIGHT_LUMA: f64 = 6.0;
pub const WEIGHT_POPULATION: f64 = 1.0;
