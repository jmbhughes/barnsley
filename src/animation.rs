use crate::{ifs::IFS, image::Image};


pub struct AnimationSequence {
    pub ifs_vec: Vec<IFS>,
    pub step_counts: Vec<usize>
}

impl AnimationSequence {
    pub fn animate(&self, width: usize, height: usize, num_iterations: usize, num_points: usize) -> Vec<Image> {
        let mut images = vec![];

        for pair_index in 0..self.ifs_vec.len()-1 {
            let num_steps_for_pair = *self.step_counts.get(pair_index).unwrap();

            let start = self.ifs_vec.get(pair_index).unwrap();
            let end = self.ifs_vec.get(pair_index + 1).unwrap();
            for step in 0..num_steps_for_pair{

                let this_ifs = start.morph(end, step as f32 / num_steps_for_pair as f32);

                let mut this_image = Image::new(width, height);
                this_ifs.evaluate(&mut this_image, num_points, num_iterations);

                images.insert(images.len(), this_image);
            }
        }
        images
    }
}

