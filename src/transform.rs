//! functions used in the IFS
//!
//! # Adding a new transform
//!
//! It's recommended to look at a simple transform's implementation before adding a new one.
//! For example, look at `LinearTransform` to understand what each part does.
//!
//! 1. Create a struct to store the transforms parameters. It should have a `base_color` and `weight` too.
//! 2. Derive `Serialize, Deserialize, Copy, Clone, Debug` for the new transform struct.
//! 3. Implement the `transform` trait for that struct.
//! 4. Add the transform to the `Transform` enum.

use crate::util::*;
use num::complex::{Complex, Complex32};
use rand::prelude::*;
use rand_distr::{Distribution, Normal};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
use std::default::Default;
use std::f32::consts::PI;
use enum_dispatch::enum_dispatch;

/// Use to map a point (x,y) to image space.
pub fn final_transform(x: f32, y: f32) -> (f32, f32) {
    let a = 0.5;
    let b = 0.0;
    let c = 0.0;
    let d = 1.0;

    let z = Complex32::new(x, y);
    let z2 = (a * z + b) / (c * z + d);
    (z2.re, z2.im)
}

#[enum_dispatch(Transformable)]
#[derive(Serialize, Deserialize, Copy, Clone, EnumIter, PartialEq)]
pub enum Transform {
    LinearTransform,
    AffineTransform,
    InverseJuliaTransform,
    MoebiusTransform
}

impl Transform {
    pub fn morph(&self, other: Transform, pct: f32) -> Transform {
        match (self, other) {
            (Transform::LinearTransform(t), Transform::LinearTransform(o)) => t.morph(&o, pct).into(),
            (Transform::MoebiusTransform(t), Transform::MoebiusTransform(o)) => t.morph(&o, pct).into(),
            (Transform::AffineTransform(t), Transform::AffineTransform(o)) => t.morph(&o, pct).into(),
            (Transform::InverseJuliaTransform(t), Transform::InverseJuliaTransform(o)) => t.morph(&o, pct).into(),
            _ => panic!("self and other must be the same transform type")
        }
    }

    pub fn index(&self) -> usize {
        match self {
            Transform::LinearTransform(_) => 0,
            Transform::AffineTransform(_) => 1,
            Transform::MoebiusTransform(_) => 2,
            Transform::InverseJuliaTransform(_) => 3
        }
    }
}

/// All transforms must have this trait
#[enum_dispatch]
pub trait Transformable {
    /// Gets the transforms base color, i.e. the color of the transform that gets repeatedly mixed
    fn get_base_color(&self) -> Color;

    /// Transform a color using the `base_color` and the `current_color`.
    fn transform_color(&self, current_color: Color) -> Color {
        let base_color = self.get_base_color();
        Color {
            r: (base_color.r + current_color.r) / 2.0,
            g: (base_color.g + current_color.g) / 2.0,
            b: (base_color.b + current_color.b) / 2.0,      
        }
    }

    /// Applies the transformation to a point
    fn transform_point(&self, point: Point) -> Point;

    /// Retrieves the transforms weight
    fn get_weight(&self) -> f32;

    /// Retrieves the name of the transformed
    fn get_name(&self) -> String;
}

pub trait Morphable<T: Transformable + ?Sized> {
    fn morph(&self, other: Box<&T>, pct: f32) -> Box<T>;
}

pub fn transform_from_str(name: String) -> Transform {
    match name.as_str() {
        "LinearTransform" => LinearTransform::random().into(),
        "AffineTransform" => AffineTransform::random().into(),
        "MoebiusTransform" => MoebiusTransform::random().into(),
        "InverseJuliaTransform" => InverseJuliaTransform::random().into(),
        _ => panic!("Not a supported transform kind")
    }
}

// LINEAR TRANSFORM
/// LinearTransform defined by the matrix:
/// [a b]
/// [c d]
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
pub struct LinearTransform {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub base_color: Color,
    pub weight: f32,
}

impl LinearTransform {
    pub fn new(a: f32, b: f32, c: f32, d: f32, base_color: Color, weight: f32) -> LinearTransform {
        LinearTransform {
            a,
            b,
            c,
            d,
            base_color,
            weight,
        }
    }

    pub fn random() -> LinearTransform {
        let mut rng = thread_rng();
        let a: f32 = rng.gen::<f32>() * 2. - 1.;
        let b: f32 = rng.gen::<f32>() * 2. - 1.;
        let c: f32 = rng.gen::<f32>() * 2. - 1.;
        let d: f32 = rng.gen::<f32>() * 2. - 1.;
        let weight: f32 = rng.gen::<f32>();
        LinearTransform {
            a,
            b,
            c,
            d,
            base_color: Color::random(),
            weight,
        }
    }

    fn morph(&self, other:&Self, pct: f32) -> Self{
       LinearTransform::new(
                    lerp_f32(self.a, other.a, pct),
                    lerp_f32(self.b, other.b, pct),
                    lerp_f32(self.c, other.c, pct),
                    lerp_f32(self.d, other.d, pct),
                    lerp_color(self.base_color, other.base_color, pct),
                    lerp_f32(self.weight, other.weight, pct))
    }
}

impl Default for LinearTransform {
    fn default() -> Self {
        LinearTransform::random()
    }
}

impl Transformable for LinearTransform {
    fn get_base_color(&self) -> Color {
        self.base_color
    }

    fn transform_point(&self, point: Point) -> Point {
        Point {
            x: self.a * point.x + self.b * point.y,
            y: self.c * point.x + self.d * point.y,
        }
    }

    fn get_weight(&self) -> f32 {
        self.weight
    }

    fn get_name(&self) -> String {
        "LinearTransform".to_string()
    }
}
impl Morphable<LinearTransform> for LinearTransform {
    fn morph(&self, other: Box<&Self>, pct: f32) -> Box<LinearTransform> where Self: Sized {
        Box::new(LinearTransform::new(
                    lerp_f32(self.a, other.a, pct),
                    lerp_f32(self.b, other.b, pct),
                    lerp_f32(self.c, other.c, pct),
                    lerp_f32(self.d, other.d, pct),
                    lerp_color(self.base_color, other.base_color, pct),
                    lerp_f32(self.weight, other.weight, pct)))
    }

}

// AFFINE TRANSFORM
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
pub struct AffineTransform {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub x_shift: f32,
    pub y_shift: f32,
    pub base_color: Color,
    pub weight: f32,
}

impl AffineTransform {
    pub fn new(
        a: f32,
        b: f32,
        c: f32,
        d: f32,
        x_shift: f32,
        y_shift: f32,
        base_color: Color,
        weight: f32,
    ) -> AffineTransform {
        AffineTransform {
            a,
            b,
            c,
            d,
            x_shift,
            y_shift,
            base_color,
            weight,
        }
    }

    pub fn random() -> AffineTransform {
        let mut rng = thread_rng();
        let a: f32 = rng.gen::<f32>() * 2. - 1.;
        let b: f32 = rng.gen::<f32>() * 2. - 1.;
        let c: f32 = rng.gen::<f32>() * 2. - 1.;
        let d: f32 = rng.gen::<f32>() * 2. - 1.;
        let x_shift: f32 = rng.gen::<f32>() * 4. - 2.;
        let y_shift: f32 = rng.gen::<f32>() * 4. - 2.;

        let normal: Normal<f64> = Normal::new(1.0, 0.15).unwrap();
        let weight: f32 = normal.sample(&mut rng) as f32;

        AffineTransform {
            a,
            b,
            c,
            d,
            x_shift,
            y_shift,
            base_color: Color::random(),
            weight,
        }
    }

   fn morph(&self, other: &Self, pct: f32) -> Self {
       AffineTransform::new(
           lerp_f32(self.a, other.a, pct),
           lerp_f32(self.b, other.b, pct),
           lerp_f32(self.c, other.c, pct),
           lerp_f32(self.d, other.d, pct),
           lerp_f32(self.x_shift, other.x_shift, pct),
           lerp_f32(self.y_shift, other.y_shift, pct),
           lerp_color(self.base_color, other.base_color, pct),
           lerp_f32(self.weight, other.weight, pct))
    }


}

impl Default for AffineTransform {
    fn default() -> Self {
        AffineTransform::random()
    }
}

impl Transformable for AffineTransform {
    fn get_base_color(&self) -> Color {
        self.base_color
    }

    fn transform_point(&self, point: Point) -> Point {
        Point {
            x: self.a * point.x + self.b * point.y + self.x_shift,
            y: self.c * point.x + self.d * point.y + self.y_shift,
        }
    }

    fn get_weight(&self) -> f32 {
        self.weight
    }

    fn get_name(&self) -> String {
        "AffineTransform".to_string()
    }
}

impl Morphable<AffineTransform> for AffineTransform {
    fn morph(&self, other: Box<&Self>, pct: f32) -> Box<Self> where Self: Sized {
        Box::new(AffineTransform::new(
            lerp_f32(self.a, other.a, pct),
            lerp_f32(self.b, other.b, pct),
            lerp_f32(self.c, other.c, pct),
            lerp_f32(self.d, other.d, pct),
            lerp_f32(self.x_shift, other.x_shift, pct),
            lerp_f32(self.y_shift, other.y_shift, pct),
            lerp_color(self.base_color, other.base_color, pct),
            lerp_f32(self.weight, other.weight, pct)))
    }
}

// MOEBIUS TRANSFORM
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
pub struct MoebiusTransform {
    pub a: Complex<f32>,
    pub b: Complex32,
    pub c: Complex32,
    pub d: Complex32,
    pub base_color: Color,
    pub weight: f32,
}

impl MoebiusTransform {
    pub fn new(
        a: Complex32,
        b: Complex32,
        c: Complex32,
        d: Complex32,
        base_color: Color,
        weight: f32,
    ) -> MoebiusTransform {
        MoebiusTransform {
            a,
            b,
            c,
            d,
            base_color,
            weight,
        }
    }

    pub fn random() -> MoebiusTransform {
        let a: Complex32 = random_complex_number();
        let b: Complex32 = random_complex_number();
        let c: Complex32 = random_complex_number();
        let d: Complex32 = random_complex_number();

        let mut rng = thread_rng();

        let normal: Normal<f64> = Normal::new(1.0, 0.15).unwrap();
        let weight: f32 = normal.sample(&mut rng) as f32;

        MoebiusTransform {
            a,
            b,
            c,
            d,
            base_color: Color::random(),
            weight,
        }
    }

    fn morph(&self, other: &Self, pct: f32) -> Self{
       MoebiusTransform::new(
                    lerp_complex32(self.a, other.a, pct),
                    lerp_complex32(self.b, other.b, pct),
                    lerp_complex32(self.c, other.c, pct),
                    lerp_complex32(self.d, other.d, pct),
                    lerp_color(self.base_color, other.base_color, pct),
                    lerp_f32(self.weight, other.weight, pct))
    }
}

impl Default for MoebiusTransform {
    fn default() -> Self {
        MoebiusTransform::random()
    }
}

impl Transformable for MoebiusTransform {
    fn get_base_color(&self) -> Color {
        self.base_color
    }

    fn transform_point(&self, point: Point) -> Point {
        let z = Complex32 {
            re: point.x,
            im: point.y,
        };
        let z2 = (self.a * z + self.b) / (self.c * z + self.d);
        Point { x: z2.re, y: z2.im }
    }

    fn get_weight(&self) -> f32 {
        self.weight
    }

    fn get_name(&self) -> String {
        "MoebiusTransform".to_string()
    }
}

impl Morphable<MoebiusTransform> for MoebiusTransform {
    fn morph(&self, other: Box<&Self>, pct: f32) -> Box<Self> where Self: Sized {
        Box::new(MoebiusTransform::new(
                    lerp_complex32(self.a, other.a, pct),
                    lerp_complex32(self.b, other.b, pct),
                    lerp_complex32(self.c, other.c, pct),
                    lerp_complex32(self.d, other.d, pct),
                    lerp_color(self.base_color, other.base_color, pct),
                    lerp_f32(self.weight, other.weight, pct)))
    }
}

// INVERSE JULIA TRANSFORM

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
pub struct InverseJuliaTransform {
    pub r: f32,
    pub theta: f32,
    pub base_color: Color,
    pub weight: f32
}

impl InverseJuliaTransform {
    pub fn new(r: f32, theta: f32, base_color: Color, weight: f32) -> InverseJuliaTransform {
        InverseJuliaTransform {
            r,
            theta,
            base_color,
            weight,
        }
    }

    pub fn random() -> InverseJuliaTransform {
        let mut rng = thread_rng();

        let r: f32 = rng.gen::<f32>().sqrt() * 0.4 + 0.8;
        let theta: f32 = 2.0 * PI * rng.gen::<f32>();

        let normal: Normal<f64> = Normal::new(1.0, 0.15).unwrap();
        let weight: f32 = normal.sample(&mut rng) as f32;

        InverseJuliaTransform::new(r, theta, Color::random(), weight)
    }

    fn morph(&self, other: &Self, pct: f32) -> Self {
        InverseJuliaTransform::new(
                    lerp_f32(self.r, other.r, pct),
                    lerp_f32(self.theta, other.theta, pct),
                    lerp_color(self.base_color, other.base_color, pct),
                    lerp_f32(self.weight, other.weight, pct))
    }
}


impl Default for InverseJuliaTransform {
    fn default() -> Self {
        InverseJuliaTransform::random()
    }
}

impl Transformable for InverseJuliaTransform {
    fn get_base_color(&self) -> Color {
        self.base_color
    }

    fn transform_point(&self, point: Point) -> Point {
        let c = Complex32::new(self.r * self.theta.cos(), self.r * self.theta.sin());

        let z = Complex32 {
            re: point.x,
            im: point.y,
        };
        let z2 = c - z;
        let new_theta = z2.im.atan2(z2.re) * 0.5;
        let sqrt_r = vec![1., -1.].choose(&mut thread_rng()).unwrap()
            * ((z2.im * z2.im + z2.re * z2.re).powf(0.25));
        Point {
            x: sqrt_r * new_theta.cos(),
            y: sqrt_r * new_theta.sin(),
        }
    }

    fn get_weight(&self) -> f32 {
        self.weight
    }

    fn get_name(&self) -> String {
        "InverseJuliaTransform".to_string()
    }
}

impl Morphable<InverseJuliaTransform> for InverseJuliaTransform {
    fn morph(&self, other: Box<&Self>, pct: f32) -> Box<Self> {
        Box::new(InverseJuliaTransform::new(
                    lerp_f32(self.r, other.r, pct),
                    lerp_f32(self.theta, other.theta, pct),
                    lerp_color(self.base_color, other.base_color, pct),
                    lerp_f32(self.weight, other.weight, pct)))
    }
}
