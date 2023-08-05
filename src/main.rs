use barnsley::transform::*;
use barnsley::ifs::IFS;
use barnsley::image::Image;


fn main() {    
    let t0: TransformEnum = MoebiusTransform::random().into();
    let t1: TransformEnum = MoebiusTransform::random().into();
    let t2: TransformEnum = InverseJuliaTransform::random().into();
    let t3: TransformEnum = InverseJuliaTransform::random().into();
    let t4: TransformEnum = AffineTransform::random().into();

    let mut my_ifs = IFS::new();
    my_ifs.add_transform(t0);
    my_ifs.add_transform(t1);
    my_ifs.add_transform(t2);
    my_ifs.add_transform(t3);
    my_ifs.add_transform(t4);

    let num_points = 10000;
    let num_iterations = 1000;

    let mut image = Image::new(512, 512);
    my_ifs.evaluate(&mut image, num_points, num_iterations);
    image.save("example_test.png", 1.max((num_points * num_iterations) / (image.height() * image.width())));
}

