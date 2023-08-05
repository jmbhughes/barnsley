use num_complex::Complex32;
use rand::prelude::*;
use rand::distributions::WeightedIndex;
use ndarray::Array3;
use image::{RgbImage, ImageBuffer, Rgb};
use rand_distr::{Normal, Distribution};

use crate::util::*;
use crate::transform::*;
use crate::image::*;

pub struct IFS {
    transforms: Vec<TransformEnum>,
    num_transforms: usize,
    total_weight: f32,
    distribution: WeightedIndex<f32>
}

impl IFS{
    pub fn new() -> IFS {
        IFS{transforms: vec![],
        num_transforms: 0,
        total_weight: 0.,
        distribution: WeightedIndex::new([1.]).unwrap()}
    }

    pub fn add_transform<'a>(&mut self, transform: TransformEnum) {
        self.total_weight += transform.get_weight();
        self.transforms.insert(self.num_transforms, transform);
        self.num_transforms += 1;
        self.distribution = WeightedIndex::new(self.transforms.iter().map(|t| t.get_weight())).unwrap(); 
    }

    fn choose_transform(&self) -> &TransformEnum {
        let mut rng = thread_rng();
        self.transforms.get(self.distribution.sample(&mut rng)).unwrap()
    }   

    pub fn evaluate(&self, image: &mut Image, num_points: usize, num_iterations: usize) {
        (0..num_points).into_iter().for_each(|v| self._single_point_evaluation(image, num_iterations))
    }

    fn _single_point_evaluation(&self, image: &mut Image, num_iterations: usize) {
        let mut rng = rand::thread_rng();

        let mut px: f32 = rng.gen::<f32>() * 2. - 1.;
        let mut py: f32 = rng.gen::<f32>() * 2. - 1.;

        let mut color = Color{r: 0.0, g: 0.0, b: 0.0};

        for _ in 0..num_iterations {
            let t = self.choose_transform();
            let new_point = t.transform_point(Point{x: px, y: py});
            px = new_point.x;
            py = new_point.y;
            color = t.transform_color(color);

            let (fx, fy) = _final_transform(px, py);
            let x = ((fx + 1.0) * (image.width() as f32 / 2.0)) as usize;
            let y = ((fy + 1.0) * (image.height() as f32 / 2.0)) as usize;
            
            image.add_radiance(x, y, color);
        }
    }
}