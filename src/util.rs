//! useful support functionality

use num::{complex::Complex32, FromPrimitive};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::{default::Default, ops::{Add, Mul}};

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

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color {
            r: (self.r + rhs.r).clamp(0.0, 1.0),
            g: (self.g + rhs.g).clamp(0.0, 1.0),
            b: (self.b + rhs.b).clamp(0.0, 1.0)
        }
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b
        }    }
}

impl FromPrimitive for Color {
    fn from_i64(n: i64) -> Option<Self> {
        Some(Color { r: (n as f32).clamp(0.0, 1.0),
            g: (n as f32).clamp(0.0, 1.0),
            b: (n as f32).clamp(0.0, 1.0)})
    }

    fn from_u64(n: u64) -> Option<Self> {
        Some(Color { r: (n as f32).clamp(0.0, 1.0),
            g: (n as f32).clamp(0.0, 1.0),
            b: (n as f32).clamp(0.0, 1.0)})
    }

    fn from_f32(n: f32) -> Option<Self> {
        Some(Color { r: n.clamp(0.0, 1.0),
            g: n.clamp(0.0, 1.0),
            b: n.clamp(0.0, 1.0)})
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
