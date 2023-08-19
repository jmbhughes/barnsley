# barnsley: Iterated Function Systems in Rust

Make your own images like this! Look at at all the variety in the [examples](examples/).

![example image](https://github.com/jmbhughes/barnsley/blob/main/examples/example4.png?raw=true)

## Make your own!
You can use [https://barnsley.dev](https://barnsley.dev) to generate your own IFS and animations right in the browser. 

This website is built in the [barnsley_gui repo](https://github.com/jmbhughes/barnsley_gui). 

## How to run crate
### Install
Rust is required to run this code. [Installation of Rust is easy though](https://www.rust-lang.org/tools/install). 
Once you have rust, install Barnsley with `cargo install barnsley`. 

Then, you can run from a template or a config.

### From a template
A template toml file specifies the image properties, the evaluation properties, and which transforms to run. 
Since you have not specified the parameters of the transforms, they're generated randomly. 
[This is an example template.](example_template.toml)
You can use the `construct` command in the program to generate a specific instance of parameters and then evaluate it
to create an image. 

1. Install with `cargo install barnsley`
2. Using Rust, run `barnsley construct example_template.toml > test.json`
3. Look at your picture in the test.png file and the configuration in test.json

### From a config
A config json file specifies the image properties, the evaluation properties, and the specific parameters of transforms. 
It can be used to recreate an image at a later date. 

1. Install with `cargo install barnsley`. 
2. Using Rust, run `barnsley construct examples/example7.json`
3. Look at the example7.png file to see the result. Note how it matches the one in the examples directory!


## Implemented transforms
- LinearTransform
- AffineTransform
- MoebiusTransform
- InverseJuliaTransform

Try different combinations of them to generate new images. 

## Origins
This code is based on [pyifs](https://github.com/jtauber/pyifs) written by [James Tauber](https://github.com/jtauber) for Python. 

The crate name is in honor of Michael Barnsley, a leading researcher of iterated function systems.
