//! representation of an non-parameterized, random IFS

use std::str::FromStr;
use serde::{Serialize, Deserialize};
use crate::transform::*;
use crate::config::*;

#[derive(Serialize, Deserialize)]
pub struct Template {
   pub image_settings: ImageSettings,
   pub evaluation_settings: EvaluationSettings,
   pub random_transforms: Vec<String>,
}

impl Template {
    pub fn generate(&self) -> Config {
        let mut transforms: Vec<TransformEnum> = vec![];
        for transform_name in self.random_transforms.clone() {
            transforms.insert(transforms.len(), TransformEnum::from_str(&transform_name).unwrap().random());
        }
        Config{image_settings: self.image_settings.clone(), evaluation_settings: self.evaluation_settings, transforms}
    }
}