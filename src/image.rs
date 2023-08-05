use ndarray::Array3;
use image::RgbImage;
use crate::util::*;

const RGB_LUMINANCE: [f32; 3] = [0.2126, 0.7152, 0.0722];
const DISPLAY_LUMINANCE_MAX: f32 = 200.0;
const SCALEFACTOR_NUMERATOR: f32 = 5.828968; // 1.219 + (DISPLAY_LUMINANCE_MAX * 0.25).powf(0.4);
const GAMMA_ENCODE: f32 = 0.45;

pub struct Image {
    data: Array3<f32>
}

impl Image {

    pub fn new(width: usize, height: usize) -> Image {
        Image {
            data: Array3::zeros((width, height, 3))
        }
    }

    pub fn width(&self) -> usize {
        self.data.shape()[0]
    }

    pub fn height(&self) -> usize {
        self.data.shape()[1]
    }

    pub fn add_radiance(&mut self, x: usize, y: usize, radiance: Color) {
        if x < self.width() && y < self.height() {
            self.data[[x, y, 0]] += radiance.r;
            self.data[[x, y, 1]] += radiance.g;
            self.data[[x, y, 2]] += radiance.b;
        }
    }

    fn calculate_scalefactor(&self, iterations: usize) -> f32 {
        let mut sum_of_logs = 0.0;

        for x in 0..self.width() {
            for y in 0..self.height() {
                let mut lum = self.data[[x, y, 0]] * RGB_LUMINANCE[0];
                lum += self.data[[x, y, 1]] * RGB_LUMINANCE[1];
                lum += self.data[[x, y, 2]] * RGB_LUMINANCE[2];
                lum /= iterations as f32;

                sum_of_logs += (lum.max(0.0001)).log10();
                // sum_of_logs += log10f32()
            }
        }
        let log_mean_luminance = 10.0f32.powf(sum_of_logs / (self.width() * self.height()) as f32);
        (SCALEFACTOR_NUMERATOR / (1.219 + log_mean_luminance.powf(0.4))).powf(2.5) / DISPLAY_LUMINANCE_MAX
    }

    fn get_gamma_corrected_pixels(&self, iterations: usize) -> Array3<f32> {
        let scalefactor = self.calculate_scalefactor(iterations);
        (self.data.clone() * scalefactor / iterations as f32).mapv(|v| v.max(0.0).powf(GAMMA_ENCODE))
    }

    pub fn save(&self, filename: &str, iterations: usize) {
        let pixels = self.get_gamma_corrected_pixels(iterations);

        let image = pixels.map(|v| ((v * 255.0 + 0.5) as u8).max(0).min(255));
        let buffer = array_to_image(image); 
        
        let _ = buffer.save(filename);

    }
}

fn array_to_image(arr: Array3<u8>) -> RgbImage {
    assert!(arr.is_standard_layout());

    let (height, width, _) = arr.dim();
    let raw = arr.into_raw_vec();

    RgbImage::from_raw(width as u32, height as u32, raw)
        .expect("container should have the right size for the image dimensions")
}
