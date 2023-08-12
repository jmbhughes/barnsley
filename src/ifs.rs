//! iterated function system
use rand::prelude::*;
use rand::distributions::WeightedIndex;
use rand_distr::Distribution;
use crate::util::*;
use crate::transform::*;
use crate::image::*;

/// Iterated function system
pub struct IFS {
    /// transforms used in the iterated function system
    transforms: Vec<TransformEnum>,
    /// the number of transforms in the IFS, stored for efficiency
    num_transforms: usize,
    /// the total weight of all the transforms in the IFS, stored for efficiency
    total_weight: f32,
    /// the distribution used in selecting a random transform, stored for efficiency instead of generating on the fly
    distribution: WeightedIndex<f32>
}

impl IFS{
    /// Define an empty IFS
    pub fn new() -> IFS {
        IFS{transforms: vec![],
        num_transforms: 0,
        total_weight: 0.,
        distribution: WeightedIndex::new([1.]).unwrap()}
    }

    /// Add a transform to the IFS
    /// 
    /// ```rust
    /// use barnsley::{transform::AffineTransform, ifs::IFS};
    /// 
    /// let mut my_ifs = IFS::new();
    /// my_ifs.add_transform(AffineTransform::random().into());
    /// ```
    pub fn add_transform<'a>(&mut self, transform: TransformEnum) {
        self.total_weight += transform.get_weight();
        self.transforms.insert(self.num_transforms, transform);
        self.num_transforms += 1;
        self.distribution = WeightedIndex::new(self.transforms.iter().map(|t| t.get_weight())).unwrap(); 
    }

    /// Select a transform at random according to the weighting 
    fn choose_transform(&self) -> &TransformEnum {
        let mut rng = thread_rng();
        self.transforms.get(self.distribution.sample(&mut rng)).unwrap()
    }   

    /// Evaluate a transform 
    /// 
    /// ```rust
    /// use barnsley::{ifs::IFS, transform::AffineTransform, image::Image};
    /// 
    /// let mut my_ifs = IFS::new();
    /// my_ifs.add_transform(AffineTransform::random().into());
    /// let mut image = Image::new(1000, 1000);
    /// my_ifs.evaluate(&mut image, 1000, 1000);
    /// ```
    pub fn evaluate(&self, image: &mut Image, num_points: usize, num_iterations: usize) {
        for _ in 0..num_points {
            self.single_point_evaluation(image, num_iterations)
        }
    }

    fn single_point_evaluation(&self, image: &mut Image, num_iterations: usize) {
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

            let (fx, fy) = final_transform(px, py);
            let x = ((fx + 1.0) * (image.width() as f32 / 2.0)) as usize;
            let y = ((fy + 1.0) * (image.height() as f32 / 2.0)) as usize;
            
            image.add_radiance(x, y, color);
        }
    }
}