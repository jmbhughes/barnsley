//! # barnsley
//! 
//! barnsley is a library and executeable for exploring iterated function systems. 
//! It's based and inspired by [pyifs](https://github.com/jtauber/pyifs) 
//! written by [James Tauber](https://github.com/jtauber) for Python. 
//! 
//! ## Running the executable
//! Simply install using `cargo install barnsley`. Run `barnsley` in a terminal to get a CLI. 
//! Each command is documented there. 
//! 
//! ## Defining an iterated function system
//! An iterated function is defined by a vector of transforms. You add them one at a time. 
//! ```rust
//! let mut ifs = IFS::new();
//! ifs.add_transform(AffineTransform::random().into());
//! 
//! let width: usize = 1000;
//! let height: usize = 1000;
//! let num_points: usize = 10000;
//! let num_iterations: usize = 1000;
//! let mut image = Image::new(width, height);
//! ifs.evaluate(&mut image, num_points, num_iterations);
//! ```
//! 
//! # Defining iterated function systems in files
//! There are two kinds of files that are used to define a IFS:
//! 1. *templates*: These are toml files specify which transforms you want to run but not their parameters. 
//!     Each time you run one you will get a different result. They're good for generating many different images. 
//! 2. *configs*: These are json files that fully specify the transforms and their parameters. You can use them 
//!     to regenerate an image at a higher resolution, change the color scheme, or explore how changing parameters impacts 
//!     the IFS. 
//! 
pub mod ifs;
pub mod image;
pub mod transform;
pub mod util;
pub mod config;
pub mod template;