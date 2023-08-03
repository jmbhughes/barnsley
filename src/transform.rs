use num_complex::Complex32;
use rand::prelude::*;
use rand_distr::{Normal, Distribution};
use crate::util::*;


pub fn _final_transform(x: f32, y: f32) -> (f32, f32) {
    let a = 0.5;
    let b = 0.0;
    let c = 0.0; 
    let d = 1.0;

    let z = Complex32::new(x, y);
    let z2 = (a * z + b) / (c * z + d); 
    return (z2.re, z2.im)
}

pub trait Transform {
    fn get_base_color(&self) -> Color;
    fn transform_color(&self, current_color: Color) -> Color {
        let base_color = self.get_base_color();
        Color{r: (base_color.r + current_color.r) / 2.0,
              g: (base_color.g + current_color.g) / 2.0,
              b: (base_color.b + current_color.b) / 2.0
        }
    }
    fn transform_point(&self, point: Point) -> Point;
    fn get_weight(&self) -> f32;
}

pub struct LinearTransform {
    a: f32,
    b: f32, 
    c: f32,
    d: f32,
    base_color: Color,
    weight: f32
}

impl LinearTransform {
    pub fn new(a: f32, b: f32, c: f32, d: f32, base_color: Color, weight: f32) -> LinearTransform {
        LinearTransform { a: a, b: b, c: c, d: d, base_color: base_color, weight: weight }
    }

    pub fn random() -> LinearTransform {
        let mut rng = rand::thread_rng();
        let a: f32 = rng.gen::<f32>() * 2. - 1.;
        let b: f32 = rng.gen::<f32>() * 2. - 1.;
        let c: f32 = rng.gen::<f32>() * 2. - 1.;
        let d: f32 = rng.gen::<f32>() * 2. - 1.;
        let weight: f32 = rng.gen::<f32>();
        LinearTransform { a, b, c, d, base_color: Color::random(), weight}
    }
}

impl Transform for LinearTransform {
    fn transform_point(&self, point: Point) -> Point {
        Point{x: self.a * point.x + self.b * point.y, 
              y: self.c * point.x + self.d * point.y}

    }

    fn get_weight(&self) -> f32 {
        self.weight
    }

    fn get_base_color(&self) -> Color {
        self.base_color
    }
}

pub struct AffineTransform{
    a: f32,
    b: f32, 
    c: f32,
    d: f32,
    xshift: f32,
    yshift: f32,
    base_color: Color,
    weight: f32
}

impl AffineTransform {
    pub fn new(a: f32, b: f32, c: f32, d: f32, xshift: f32, yshift: f32, base_color: Color, weight: f32) -> AffineTransform {
        AffineTransform { a: a, b: b, c: c, d: d, xshift: xshift, yshift: yshift, base_color: base_color, weight: weight }
    }

    pub fn random() -> AffineTransform {
        let mut rng = rand::thread_rng();
        let a: f32 = rng.gen::<f32>() * 2. - 1.;
        let b: f32 = rng.gen::<f32>() * 2. - 1.;
        let c: f32 = rng.gen::<f32>() * 2. - 1.;
        let d: f32 = rng.gen::<f32>() * 2. - 1.;
        let xshift: f32 = rng.gen::<f32>() * 4. - 2.;
        let yshift: f32 = rng.gen::<f32>() * 4. - 2.;

        let normal: Normal<f64> = Normal::new(1.0, 0.15).unwrap();
        let weight: f32 = normal.sample(&mut rng) as f32;

        AffineTransform { a, b, c, d, xshift, yshift, base_color: Color::random(), weight}
    }
}

impl Transform for AffineTransform {
    fn transform_point(&self, point: Point) -> Point {
        Point{x: self.a * point.x + self.b * point.y + self.xshift, 
              y: self.c * point.x + self.d * point.y + self.yshift}

    }

    fn get_weight(&self) -> f32 {
        self.weight
    }

    fn get_base_color(&self) -> Color {
        self.base_color
    }
}

pub struct MoebiusTransform{
    a: Complex32,
    b: Complex32, 
    c: Complex32,
    d: Complex32,
    base_color: Color,
    weight: f32
}

impl MoebiusTransform {
    pub fn new(a: Complex32, b: Complex32, c: Complex32, d: Complex32, base_color: Color, weight: f32) -> MoebiusTransform {
        MoebiusTransform { a: a, b: b, c: c, d: d, base_color: base_color, weight: weight }
    }

    pub fn random() -> MoebiusTransform {

        let a: Complex32 = random_complex_number();
        let b: Complex32 = random_complex_number();
        let c: Complex32 = random_complex_number();
        let d: Complex32 = random_complex_number();

        let mut rng = rand::thread_rng();

        let normal: Normal<f64> = Normal::new(1.0, 0.15).unwrap();
        let weight: f32 = normal.sample(&mut rng) as f32;

        MoebiusTransform { a, b, c, d, base_color: Color::random(), weight}
    }
}

impl Transform for MoebiusTransform {
    fn transform_point(&self, point: Point) -> Point {
        let z = Complex32{re: point.x, im: point.y};
        let z2 = (self.a * z + self.b) / (self.c * z + self.d);
        Point{x:z2.re, y: z2.im}
    }

    fn get_weight(&self) -> f32 {
        self.weight
    }

    fn get_base_color(&self) -> Color {
        self.base_color
    }
}