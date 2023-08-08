# barnsley: Iterated Function Systems in Rust

Make your own images like this! Look at at all the variety in the [examples](examples/).

![example image](https://github.com/jmbhughes/barnsley/blob/main/examples/example4.png?raw=true)

This code is based on [pyifs](https://github.com/jtauber/pyifs) written by [James Tauber](https://github.com/jtauber) for Python. 

## How to run
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

## Coming soon
- [x] the base package 
- [x] a configurable main function 
- [ ] more color schemes
- [ ] more transforms
- [ ] a mathematical guide
- [ ] classic fractals
- [ ] animation of the ifs executing
