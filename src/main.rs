#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use barnsley::config::*;
use barnsley::ifs::IFS;
use barnsley::image::Image;
use barnsley::template::*;
use barnsley::transform::{AffineTransform, MoebiusTransform};
use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::Read;
use serde_json;
use toml;
use eframe::egui;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generates a config from a template
    Generate { template_path: String },
    /// Evaluates a config file
    Evaluate { config_path: String},
    /// Generates a config from a template and evaluates it, combo of generate and evaluate
    Construct { template_path: String},
    /// Runs the GUI
    GUI
}

fn load_template(template_path: &String) -> Template{
    let mut file = File::open(template_path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    toml::from_str(&data).unwrap()

}

fn load_config(config_path: &String) -> Config {
    let mut file = File::open(config_path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    serde_json::from_str(&data).unwrap()

}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Generate { template_path } => {
            let config = load_template(template_path).generate();
            println!("{}", serde_json::to_string(&config).unwrap());
        },
        Commands::Evaluate { config_path } => {
            let config = load_config(config_path);
            config.run();
        },
        Commands::Construct { template_path } => {
            let config = load_template(template_path).generate();
            println!("{}", serde_json::to_string(&config).unwrap());        
            config.run();
        },
        Commands::GUI => {
            run_gui();
        }
    }
}



fn run_gui() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1200.0, 800.0)),
        ..Default::default()
    };

    let width = 500;
    let height = 500;
    let mut my_image = Image::new(width, height);

    let mut my_ifs = IFS::new();
    my_ifs.add_transform(AffineTransform::random().into());
    my_ifs.add_transform(MoebiusTransform::random().into());

    let num_points = 1000;
    let num_iterations = 1000;
    my_ifs.evaluate(&mut my_image, num_points, num_iterations);
    // let data = my_image.to_u8(1.max((num_points * num_iterations) / (my_image.height() * my_image.width()))).as_slice().unwrap().to_owned();

    eframe::run_simple_native("Barnsley", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Barnsley test"); 
            
            let texture: &egui::TextureHandle = { 
                // Load the texture only once.
                &ui.ctx().load_texture(
                    "my-image",
                    egui::ColorImage::from_rgb([width, height], my_image.to_u8(1.max((num_points * num_iterations) / (my_image.height() * my_image.width()))).as_slice().unwrap()),
                    Default::default()
                )
            };

            ui.image(texture, texture.size_vec2());

            if ui.button("Click me").clicked() {
                my_image.clear();
                my_ifs = IFS::new();
                my_ifs.add_transform(AffineTransform::random().into());
                my_ifs.add_transform(MoebiusTransform::random().into());
                my_ifs.evaluate(&mut my_image, num_points, num_iterations);

            }
        });
    })
}

struct MyImage {
    texture: Option<egui::TextureHandle>,
}

impl MyImage {
    fn ui(&mut self, ui: &mut egui::Ui) {
        let texture: &egui::TextureHandle = self.texture.get_or_insert_with(|| {
            // Load the texture only once.
            ui.ctx().load_texture(
                "my-image",
                egui::ColorImage::example(),
                Default::default()
            )
        });

        // Show the image:
        ui.image(texture, texture.size_vec2());
    }
}