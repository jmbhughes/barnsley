use rand::prelude::*;
use num_complex::Complex32;
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Color {
    pub r: f32, 
    pub g: f32, 
    pub b: f32
}

impl Color {
    pub fn random() -> Color {
        let mut rng = rand::thread_rng();
        Color {
            r: rng.gen::<f32>(),
            g: rng.gen::<f32>(),
            b: rng.gen::<f32>()
        }

    }
}

pub struct Point {
    pub x: f32,
    pub y: f32
}

pub fn random_complex_number() -> Complex32 {
    let mut rng = rand::thread_rng();
    Complex32{re:rng.gen::<f32>() * 2. - 1., im:rng.gen::<f32>() * 2. - 1.}
}