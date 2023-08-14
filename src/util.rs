//! useful support functionality

use num::complex::Complex32;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::default::Default;


pub enum Parameter {
    F32(f32),
    Complex32(Complex32)
}

pub fn lerp(a: Parameter, b: Parameter, pct: f32) -> Parameter {
    match (a, b) {
        (Parameter::F32(a), Parameter::F32(b)) => Parameter::F32(lerp_f32(a, b, pct)),
        (Parameter::Complex32(a), Parameter::Complex32(b)) => Parameter::Complex32(lerp_complex32(a, b, pct)),
        _ => panic!("a and b must have the same type to lerp.")
    }
}

/// lerp between two floats
pub fn lerp_f32(a: f32, b: f32, pct: f32) -> f32 {
    a * (1.0 - pct) + b * pct
}

/// lerp between two complex
pub fn lerp_complex32(a: Complex32, b: Complex32, pct: f32) -> Complex32 {
    Complex32::new(lerp_f32(a.re, b.re, pct), lerp_f32(a.re, b.re, pct))
}

/// lerp between two colors
pub fn lerp_color(a: Color, b: Color, pct: f32) -> Color {
    Color {
        r: lerp_f32(a.r, b.r, pct),
        g: lerp_f32(a.g, b.g, pct),
        b: lerp_f32(a.b, b.b, pct),
    }
}

/// representation of an RGB color
#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    /// generates a random color
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Color {
            r: rng.gen::<f32>(),
            g: rng.gen::<f32>(),
            b: rng.gen::<f32>(),
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
    pub y: f32,
}

/// generates a random complex number
pub fn random_complex_number() -> Complex32 {
    let mut rng = rand::thread_rng();
    Complex32 {
        re: rng.gen::<f32>() * 2. - 1.,
        im: rng.gen::<f32>() * 2. - 1.,
    }
}
