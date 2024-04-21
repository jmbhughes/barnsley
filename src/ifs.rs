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
    pub transforms: Vec<Transform>,
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

    /// Update the IFS to new random weights and random parameters for each transform
    pub fn randomize(&mut self) {
        self.transforms = self.transforms.iter().map(|t| transform_from_str(t.get_name())).collect();
        self.distribution = WeightedIndex::new(self.transforms.iter().map(|t| t.get_weight())).unwrap(); 
        self.total_weight = self.transforms.iter().map(|t| t.get_weight()).sum();
    }

    /// Get the number of transforms in an IFS
    pub fn len(&self) -> usize {
        self.num_transforms
    }

    /// True if there are no transforms in the IFS, False otherwise
    pub fn is_empty(&self) -> bool {
        self.num_transforms == 0
    }

    /// Get the transform at index i
    pub fn get_transform(&self, i: usize) -> Transform {
        if i < self.len() {
            *self.transforms.get(i).unwrap()
        } else {
            panic!("i is greater than the number of transforms")
        }
    }

    /// Check that all the transforms in one IFS are the
    /// same as another IFS (values can be different)
    pub fn check_transforms_match(&self, other: &Self) -> bool {
        if self.transforms.len() != other.transforms.len() {
            false
        } else {
            for i in 0..self.transforms.len() {
                if self.transforms.get(i).unwrap().get_name() != other.transforms.get(i).unwrap().get_name() {
                    return false
                }
            }
            true
        }
    }

    /// Add a transform to the IFS
    /// 
    /// ```rust
    /// use barnsley::{transform::AffineTransform, ifs::IFS};
    /// 
    /// let mut my_ifs = IFS::new();
    /// my_ifs.add_transform(AffineTransform::random().into());
    /// ```
    pub fn add_transform(&mut self, transform: Transform) {
        self.total_weight += transform.get_weight();
        self.transforms.insert(self.num_transforms, transform);
        self.num_transforms += 1;
        self.distribution = WeightedIndex::new(self.transforms.iter().map(|t| t.get_weight())).unwrap(); 
    }

    /// Remove a transform from an IFS
    pub fn delete_transform(&mut self, index: usize) {
        let transform = self.transforms.get(index).unwrap();
        self.total_weight -= transform.get_weight();
        self.num_transforms -= 1;
        self.transforms.remove(index);
        self.distribution = WeightedIndex::new(self.transforms.iter().map(|t| t.get_weight())).unwrap(); 
    }

    /// Select a transform at random according to the weighting 
    fn choose_transform(&self) -> &Transform {
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

    pub fn morph(&self, other: &Self, pct: f32) -> Self {
           if !self.check_transforms_match(other) {
               panic!("Transforms must match");
           } else {
               let mut out = IFS::new();
               for i in 0..self.transforms.len() {
                    let a = *self.transforms.get(i).unwrap();
                    let b = *other.transforms.get(i).unwrap();
                    let new = a.morph(b, pct);
                    out.add_transform(new);
               }
               out
           }
   }
}

impl Default for IFS {
    fn default() -> Self {
        Self::new()
    }
}