use num_complex::Complex32;
use rand::prelude::*;
use rand::distributions::WeightedIndex;
use ndarray::Array3;
use image::{RgbImage, ImageBuffer, Rgb};
use rand_distr::{Normal, Distribution};


#[derive(Copy, Clone)]
pub struct Color {
    r: f32, 
    g: f32, 
    b: f32
}

impl Color {
    fn random() -> Color {
        let mut rng = rand::thread_rng();
        Color {
            r: rng.gen::<f32>() * 255.,
            g: rng.gen::<f32>() * 255.,
            b: rng.gen::<f32>() * 255.
        }

    }
}

pub struct Point {
    x: f32,
    y: f32
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

struct LinearTransform {
    a: f32,
    b: f32, 
    c: f32,
    d: f32,
    base_color: Color,
    weight: f32
}

impl LinearTransform {
    fn new(a: f32, b: f32, c: f32, d: f32, base_color: Color, weight: f32) -> LinearTransform {
        LinearTransform { a: a, b: b, c: c, d: d, base_color: base_color, weight: weight }
    }

    fn random() -> LinearTransform {
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
    fn new(a: f32, b: f32, c: f32, d: f32, xshift: f32, yshift: f32, base_color: Color, weight: f32) -> AffineTransform {
        AffineTransform { a: a, b: b, c: c, d: d, xshift: xshift, yshift: yshift, base_color: base_color, weight: weight }
    }

    fn random() -> AffineTransform {
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

pub struct IFS {
    transforms: Vec<Box< dyn Transform + Sync>>,
    num_transforms: usize,
    total_weight: f32
}

pub struct Image {
    data: Array3<f32>
}

const RGB_LUMINANCE: [f32; 3] = [0.2126, 0.7152, 0.0722];
const DISPLAY_LUMINANCE_MAX: f32 = 200.0;
const SCALEFACTOR_NUMERATOR: f32 = 6.0007625; // 1.219 + (DISPLAY_LUMINANCE_MAX * 0.25).powf(0.4);
const GAMMA_ENCODE: f32 = 0.45;

impl Image {

    fn new(width: usize, height: usize) -> Image {
        Image {
            data: Array3::zeros((width, height, 3))
        }
    }

    fn mean(&self) -> f32 {
        self.data.mean().unwrap()
    }

    fn to_u8(&self) -> Array3<u8> {
        self.data.map(|&v| v as u8)
    }

    fn width(&self) -> usize {
        self.data.shape()[0]
    }

    fn height(&self) -> usize {
        self.data.shape()[1]
    }

    fn add_radiance(&mut self, x: usize, y: usize, radiance: Color) {
        if 0 <= x && x < self.width() && 0 <= y && y < self.height() {
            self.data[[x, y, 0]] += radiance.r;
            self.data[[x, y, 1]] += radiance.g;
            self.data[[x, y, 2]] += radiance.b;
        }
    }

    fn calculate_scalefactor(&self, iterations: usize) -> f32 {
        let mut sum_of_logs = 0.0;

        let width = self.data.shape()[0];
        let height = self.data.shape()[1];
        for x in 0..width {
            for y in 0..height {
                let mut lum = self.data[[x, y, 0]] * RGB_LUMINANCE[0];
                lum += self.data[[x, y, 1]] * RGB_LUMINANCE[1];
                lum += self.data[[x, y, 2]] * RGB_LUMINANCE[2];
                lum /= iterations as f32;

                sum_of_logs += lum.log10().max(0.0001);
                // sum_of_logs += log10f32()
            }
        }
        let log_mean_luminance = 10.0f32.powf(sum_of_logs / (width * height) as f32);
        (SCALEFACTOR_NUMERATOR / (1.219 + log_mean_luminance.powf(0.4)).powf(2.5)) / DISPLAY_LUMINANCE_MAX
    }

    fn get_gamma_corrected_pixels(&self, iterations: usize) -> Array3<f32> {
        let scalefactor = self.calculate_scalefactor(iterations);
        let mut pixels : Array3<f32>= Array3::zeros((self.width(), self.height(), 3));
        (self.data.clone() * scalefactor / iterations as f32).mapv(|v| v.max(0.0).powf(GAMMA_ENCODE))
    }

    fn save(&self, filename: &str, iterations: usize) {
        let pixels = self.get_gamma_corrected_pixels(iterations);

        let image = pixels.map(|v| ((v * 255.0 + 0.5) as u8).max(0).min(255));
        let buffer = array_to_image(image); 
        
        buffer.save(filename);
    }
}

impl IFS{
    fn new() -> IFS {
        IFS{transforms: vec![],
        num_transforms: 0,
        total_weight: 0.}
    }

    fn add_transform<'a>(&mut self, transform: Box<dyn Transform + Sync>) {
        self.total_weight += transform.get_weight();
        self.transforms.insert(self.num_transforms, transform);
        self.num_transforms += 1;
    }

    fn choose_transform(&self) -> &Box<dyn Transform + Sync> {
        let mut rng = thread_rng();
        let distribution = WeightedIndex::new(self.transforms.iter().map(|t| t.get_weight())).unwrap(); 
        self.transforms.get(distribution.sample(&mut rng)).unwrap()
    }   


    fn evaluate(&self, image: &mut Image, num_points: usize, num_iterations: usize) {
        (0..num_points).into_iter().for_each(|v| self._single_point_evaluation(image, num_iterations))
    }

    fn _single_point_evaluation(&self, image: &mut Image, num_iterations: usize) {
        let mut rng = rand::thread_rng();

        let mut x: f32 = rng.gen::<f32>() * 2. - 1.;
        let mut y: f32 = rng.gen::<f32>() * 2. - 1.;

        let mut color = Color{r: 0.0, g: 0.0, b: 0.0};

        for _ in 0..num_iterations {
            let t = self.choose_transform();
            let new_point = t.transform_point(Point{x: x, y: y});
            x = new_point.x;
            y = new_point.y;
            color = t.transform_color(color);

            (x, y) = _final_transform(x, y);
            let x = ((x + 1.0) * (image.width() as f32 / 2.0)) as usize;
            let y = ((y + 1.0) * (image.height() as f32 / 2.0)) as usize;
            
            image.add_radiance(x, y, color);
        }
    }
}

fn _final_transform(x: f32, y: f32) -> (f32, f32) {
    let a = 0.5;
    let b = 0.0;
    let c = 0.0; 
    let d = 1.0;

    let z = Complex32::new(x, y);
    let z2 = (a * z + b) / (c * z + d); 
    return (z2.re, z2.im)
}

fn array_to_image(arr: Array3<u8>) -> RgbImage {
    assert!(arr.is_standard_layout());

    let (height, width, _) = arr.dim();
    let raw = arr.into_raw_vec();

    RgbImage::from_raw(width as u32, height as u32, raw)
        .expect("container should have the right size for the image dimensions")
}


fn main() {
    let mut rng = rand::thread_rng();
    let mut a: f32 = rng.gen::<f32>() * 2. - 1.;
    let mut b: f32 = rng.gen::<f32>() * 2. - 1.;
    let mut c: f32 = rng.gen::<f32>() * 2. - 1.;
    let mut d: f32 = rng.gen::<f32>() * 2. - 1.;
    
    let t0 = Box::new(AffineTransform::random());
    let t1 = Box::new(AffineTransform::random());
    let t2 = Box::new(AffineTransform::random());
    let t3 = Box::new(AffineTransform::random());
    let t4 = Box::new(AffineTransform::random());
    let t5 = Box::new(AffineTransform::random());
    let t6 = Box::new(AffineTransform::random());
    let t7 = Box::new(AffineTransform::random());
    let t8 = Box::new(AffineTransform::random());
    let t9 = Box::new(AffineTransform::random());

    let mut my_ifs = IFS::new();
    my_ifs.add_transform(t0);
    my_ifs.add_transform(t1);
    my_ifs.add_transform(t2);
    my_ifs.add_transform(t3);
    my_ifs.add_transform(t4);
    my_ifs.add_transform(t5);
    my_ifs.add_transform(t6);
    my_ifs.add_transform(t7);
    my_ifs.add_transform(t8);
    my_ifs.add_transform(t9);

    let num_points = 10000;
    let num_iterations = 1000;

    let mut image = Image::new(512, 512);
    my_ifs.evaluate(&mut image, num_points, num_iterations);
    image.save("out.png", num_iterations);
}
