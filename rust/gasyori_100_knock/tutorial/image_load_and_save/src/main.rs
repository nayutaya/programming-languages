use std::path::Path;

fn main() {
    let input_image_path = Path::new("../../data/lenna.png");
    let mut image_buffer = image::open(input_image_path).unwrap().to_rgba();

    let output_image_path = Path::new("out.png");
    image_buffer.save(output_image_path);
}
