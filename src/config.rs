use toml;
use serde::{Serialize, Deserialize};
use crate::transform::*;


#[derive(Serialize, Deserialize)]
struct Config {
   image_settings: ImageSettings,
   evaluation_settings: EvaluationSettings,
   tranform_serializations: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct ImageSettings {
   width: u32,
   height: u32,
   path: String
}

#[derive(Serialize, Deserialize)]
struct EvaluationSettings {
   iterations: u32,
   num_points: u32,
}

pub fn test() {
    let my_config: Config = Config {
        image_settings: ImageSettings { width: 512, height: 512, path: "test.png".to_string() },
        evaluation_settings: EvaluationSettings { iterations: 10000, num_points: 1000 },
        tranform_serializations: vec!["test".to_string()] // LinearTransform::random().to_string()]
    };

    let serialized_config: String = toml::to_string(&my_config).unwrap();
    println!("{}", serialized_config);
}