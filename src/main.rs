use core::num;

use rand::prelude::*;
use rand::distributions::WeightedIndex;
use ndarray::Array3;

pub struct Color {
    r: f32, 
    g: f32, 
    b: f32
}

pub struct Point {
    x: f32,
    y: f32
}


pub trait Transform {
    fn transform_point(&self, point: Point) -> Point;
    fn get_weight(&self) -> f32;
}

struct LinearTransform {
    a: f32,
    b: f32, 
    c: f32,
    d: f32
}

impl Transform for LinearTransform {
    fn transform_point(&self, point: Point) -> Point {
        Point{x: self.a * point.x + self.b * point.y, 
              y: self.c * point.x + self.d * point.y}

    }

    fn get_weight(&self) -> f32 {
        1.0
    }
}

pub struct IFS {
    transforms: Vec<Box< dyn Transform>>,
    num_transforms: usize,
    total_weight: f32
}

impl IFS{
    fn new() -> IFS {
        IFS{transforms: vec![],
        num_transforms: 0,
        total_weight: 0.}
    }

    fn add_transform<'a>(&mut self, transform: Box<dyn Transform>) {
        self.total_weight += transform.get_weight();
        self.transforms.insert(self.num_transforms, transform);
        self.num_transforms += 1;
    }

    fn choose_transform(&self) -> &Box<dyn Transform> {
        let mut rng = thread_rng();
        let distribution = WeightedIndex::new(self.transforms.iter().map(|t| t.get_weight())).unwrap(); 
        self.transforms.get(distribution.sample(&mut rng)).unwrap()
    }   

    fn evaluate(&self, mut image: Array3<f32>, num_points: usize, num_iterations: usize) {
        let width = image.shape()[0];
        let height = image.shape()[1];

        let mut rng = rand::thread_rng();

        for _ in 0..num_points {
            let mut x: f32 = rng.gen::<f32>() * 2. - 1.;
            let mut y: f32 = rng.gen::<f32>() * 2. - 1.;

            for _ in 0..num_iterations {
                let t = self.choose_transform();
                let new_point = t.transform_point(Point{x: x, y: y});
                x = new_point.x;
                y = new_point.y;
                println!("{} {}", x, y);
            }
        }
        // width, height = image.width, image.height
        // for i in range(num_points): 
        //     px = random.uniform(-1, 1)
        //     py = random.uniform(-1, 1)
        //     r, g, b = 0.0, 0.0, 0.0
    
        //     for j in range(iterations):
        //         t = self._choose_transform()
        //         px, py = t.transform(px, py)
        //         r, g, b = t.transform_colour(r, g, b)
                
        //         fx, fy = self._final_transform(px, py)
        //         x = int((fx + 1) * width / 2)
        //         y = int((fy + 1) * height / 2)
        //         image.add_radiance(x, y, [r, g, b])
        // return image
    }
}
fn main() {
    let t = Box::new(LinearTransform{a: 1., b: 2., c: 3., d: 4.});
    let mut my_ifs = IFS::new();
    my_ifs.add_transform(t);

    let image = Array3::zeros((100, 100, 3));
    my_ifs.evaluate(image, 100, 5);
}
