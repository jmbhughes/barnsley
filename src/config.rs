//! core definition for an IFS run that can be serialized to a file
use serde::{Serialize, Deserialize};
use crate::transform::*;
use crate::ifs::*;
use crate::image::Image;

/// Configs are used to define an IFS run: the image settings used, the evaluation settings, and the transforms. 
#[derive(Serialize, Deserialize)]
pub struct Config {
   pub image_settings: ImageSettings,
   pub evaluation_settings: EvaluationSettings,
   pub transforms: Vec<Transforms>
}

impl Config{
   pub fn run(self) {
      let mut ifs = IFS::new();

      for transform in self.transforms.into_iter() {
         ifs.add_transform(transform);
      }

    let num_points = self.evaluation_settings.num_points as usize;
    let num_iterations = self.evaluation_settings.num_iterations as usize;

    let mut image = Image::new(self.image_settings.width as usize, self.image_settings.height as usize);
    ifs.evaluate(&mut image, num_points, num_iterations);
    image.save(&self.image_settings.path, 1.max((num_points * num_iterations) / (image.height() * image.width())));
   }
}

/// Configuration of the image in an IFS run
#[derive(Serialize, Deserialize, Clone)]
pub struct ImageSettings {
   /// how wide in pixels the generated image will be
   pub width: u32,
   /// how tall in pixels the generated image will be    
   pub height: u32,
   /// where to save the image
   pub path: String
}

/// Configuration of the evaluation of an IFS run
#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct EvaluationSettings {
   /// how many iterations each point is evaluated for
   pub num_iterations: u32,
   /// how many points are passed through the IFS
   pub num_points: u32,
}
