//! useful support functionality

use rand::prelude::*;
use num::complex::Complex32;
use serde::{Serialize, Deserialize};
use std::default::Default;

/// representation of an RGB color
#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct Color {
    pub r: f32, 
    pub g: f32, 
    pub b: f32
}

impl Color {
    /// generates a random color
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Color {
            r: rng.gen::<f32>(),
            g: rng.gen::<f32>(),
            b: rng.gen::<f32>()
        }

    }
}

impl Default for Color {
    fn default() -> Self {
        Color::random()
    }
}

/// a two-dimensional point
pub struct Point {
    pub x: f32,
    pub y: f32
}

/// generates a random complex number
pub fn random_complex_number() -> Complex32 {
    let mut rng = rand::thread_rng();
    Complex32{re:rng.gen::<f32>() * 2. - 1., im:rng.gen::<f32>() * 2. - 1.}
}