use serde::{Serialize, Deserialize};
use crate::transform::*;
use crate::ifs::*;
use crate::image::Image;

#[derive(Serialize, Deserialize)]
pub struct Config {
   pub image_settings: ImageSettings,
   pub evaluation_settings: EvaluationSettings,
   pub transforms: Vec<TransformEnum>,
}

impl Config {
   pub fn run(&self) {
      let mut ifs = IFS::new();
      for tranform in self.transforms.iter() {
         ifs.add_transform(*tranform);
      }

    let num_points = self.evaluation_settings.num_points as usize;
    let num_iterations = self.evaluation_settings.num_iterations as usize;

    let mut image = Image::new(self.image_settings.width as usize, self.image_settings.height as usize);
    ifs.evaluate(&mut image, num_points, num_iterations);
    image.save(&self.image_settings.path, 1.max((num_points * num_iterations) / (image.height() * image.width())));
   }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ImageSettings {
   pub width: u32,
   pub height: u32,
   pub path: String
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct EvaluationSettings {
   pub num_iterations: u32,
   pub num_points: u32,
}
