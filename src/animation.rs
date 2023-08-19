//! a struct and associated methods for animations
//! 
//! An animation sequence takes `n` iterated function systems and `n-1` step counts.
//! Step counts indicate how many interpolations happen between a pair of IFSes. 
//! For example, if the `ifs_vec = vec![a, b, c];`
//! and `step_counts=vec![100, 200];` It would take 100 steps interpolating between `a` and `b` 
//! and `200` steps between `b` and `c`. 
//! 
use crate::{ifs::IFS, image::Image};

/// Representation of animation.
pub struct AnimationSequence {
    pub ifs_vec: Vec<IFS>,
    pub step_counts: Vec<usize>
}

impl AnimationSequence {
    fn determine_current_pair_index(&self, current_step: usize) -> usize {
        let mut pair_index = 0;
        let mut accumulator = 0;
        while accumulator < current_step {
            accumulator += self.step_counts.get(pair_index).expect("current_step exceeds total step count.");
            pair_index += 1;
        }
        pair_index - 1
    }

    fn determine_current_pct(&self, current_step: usize, pair_index: usize) -> f32{
        let mut current_pair = 0;
        let mut accumulator = 0;
        while current_pair < pair_index {
            accumulator += self.step_counts.get(current_pair).expect("current_step exceeds total step count.");
            current_pair += 1;
        }
        let this_pair_steps = current_step - accumulator;
        this_pair_steps as f32 / *self.step_counts.get(pair_index).unwrap() as f32
    }

    /// Animate to return only one image at a given `current_step`.
    /// ```rust
    /// use barnsley::{transform::AffineTransform, ifs::IFS, animation::AnimationSequence};
    /// 
    /// let mut start = IFS::new();
    /// start.add_transform(AffineTransform::random().into());
    /// 
    /// let mut middle = IFS::new();
    /// middle.add_transform(AffineTransform::random().into());
    /// 
    /// let mut end = IFS::new();
    /// end.add_transform(AffineTransform::random().into());
    /// 
    /// let animation = AnimationSequence{ifs_vec: vec![start, middle, end], step_counts: vec![10, 20]};
    /// let result = animation.animate_single_step(100, 100, 100, 100, 11);
    /// ```
    pub fn animate_single_step(&self, width: usize, height: usize, 
        num_iterations: usize, num_points: usize, current_step: usize) -> Image {
            let pair_index = self.determine_current_pair_index(current_step);

            let start = self.ifs_vec.get(pair_index).unwrap();
            let end = self.ifs_vec.get(pair_index + 1).unwrap();
            let pct = self.determine_current_pct(current_step, pair_index); 
            let this_ifs = start.morph(end, pct);

            let mut this_image = Image::new(width, height);
            this_ifs.evaluate(&mut this_image, num_points, num_iterations);
            this_image
    }

    /// Animate all steps in an IFS to get a `Vec<Image>`.
    /// ```rust
    /// use barnsley::{transform::AffineTransform, ifs::IFS, animation::AnimationSequence};
    /// 
    /// let mut start = IFS::new();
    /// start.add_transform(AffineTransform::random().into());
    /// 
    /// let mut middle = IFS::new();
    /// middle.add_transform(AffineTransform::random().into());
    /// 
    /// let mut end = IFS::new();
    /// end.add_transform(AffineTransform::random().into());
    /// 
    /// let animation = AnimationSequence{ifs_vec: vec![start, middle, end], step_counts: vec![2, 3]};
    /// let movie = animation.animate(100, 100, 100, 100);
    /// ```
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

#[cfg(test)]
mod tests {
    use crate::{ifs::IFS, animation::AnimationSequence};

    #[test]
    fn test_determine_current_pair_index() {
        let ifs1: IFS = IFS::new();
        let ifs2: IFS = IFS::new();
        let ifs3: IFS = IFS::new();
        let ifs4: IFS = IFS::new();

        let seq = AnimationSequence{ifs_vec: vec![ifs1, ifs2, ifs3, ifs4], 
            step_counts: vec![100, 200, 300]};

        assert_eq!(seq.determine_current_pair_index(30), 0);
        assert_eq!(seq.determine_current_pair_index(100), 0);
        assert_eq!(seq.determine_current_pair_index(101), 1);
        assert_eq!(seq.determine_current_pair_index(101), 1);
        assert_eq!(seq.determine_current_pair_index(305), 2);
    }

    #[test]
    fn test_determine_current_pct() {
        let ifs1: IFS = IFS::new();
        let ifs2: IFS = IFS::new();
        let ifs3: IFS = IFS::new();
        let ifs4: IFS = IFS::new();

        let seq = AnimationSequence{ifs_vec: vec![ifs1, ifs2, ifs3, ifs4], 
            step_counts: vec![100, 200, 300]};

        assert!((seq.determine_current_pct(30, 0) - 0.3).abs() < 0.01);
        assert!((seq.determine_current_pct(101, 1) - 1.0/200.0).abs() < 0.01);
        assert!((seq.determine_current_pct(201, 1) - 101.0/200.0).abs() < 0.01);
    }
}