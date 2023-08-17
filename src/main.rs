#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use barnsley::config::*;
use barnsley::template::*;
use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::Read;
use serde_json;
use toml;

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
    Construct { template_path: String}
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
        }
    }
}
