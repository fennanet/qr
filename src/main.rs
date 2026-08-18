use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use png;



fn main() {
    let colors = "bwbwbw";
    encode_png(3, 2, generate_color_data(colors));

}

fn generate_color_data(colors: &str) -> Vec<u8>{
    let mut data: Vec<u8> = Vec::new();
    for color in colors.chars() {
        if color == 'b' {
            data.push(0);
            data.push(0);
            data.push(0);
            data.push(255);
        }
        else if color == 'w' {
            data.push(255);
            data.push(255);
            data.push(255);
            data.push(255);
        }
        else {
            println!("Invalid color: {}", color);
        }
    }
    data
}


fn encode_png(width: u32, height: u32, data: Vec<u8>) {
    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    
    let mut encoder = png::Encoder::new(w, width, height); // Width is 3 pixels and height is 2.
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));     // 1.0 / 2.2, unscaled, but rounded
    let source_chromaticities = png::SourceChromaticities::new(     // Using unscaled instantiation here
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000)
    );
    encoder.set_source_chromaticities(source_chromaticities);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&data).unwrap();
}