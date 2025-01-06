use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

fn main() {
    const IMAGE_WIDTH: usize = 256;
    const IMAGE_HEIGHT: usize = 256;

    let path = Path::new(r"./test.png");
    let file = File::create(path).unwrap();
    let w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();

    let mut image_data = [0_u8; IMAGE_HEIGHT * IMAGE_WIDTH * 4];
    for i in 0..IMAGE_HEIGHT {
        for j in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH as f64 - 1.0_f64);
            let g = j as f64 / (IMAGE_HEIGHT as f64 - 1.0_f64);
            let b = 0.0_f64;
            let a = 1.0_f64;

            let int_conversion = 255.99_f64;
            let ri = (int_conversion * r) as u8;
            let gi = (int_conversion * g) as u8;
            let bi = (int_conversion * b) as u8;
            let ai = (int_conversion * a) as u8;
            // println!("{} {} {} {}", ri, gi, bi, ai);
            image_data[4 * (i * IMAGE_HEIGHT + j) + 0] = ri;
            image_data[4 * (i * IMAGE_HEIGHT + j) + 1] = gi;
            image_data[4 * (i * IMAGE_HEIGHT + j) + 2] = bi;
            image_data[4 * (i * IMAGE_HEIGHT + j) + 3] = ai;
        }
    }
    writer.write_image_data(&image_data).unwrap();
}
