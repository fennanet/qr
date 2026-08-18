use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use png;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    ///how much to zoom in on each pixel
    #[arg(short, long, default_value = "100")]
    zoom: u16,
}

fn main() {
    let colors: Vec<&[&str]> = vec![
        &["b", "w", "b", "w"],
        &["w", "b", "w", "b"],
        &["b", "w", "b", "w"],
        &["w", "b", "w", "b"],
    ];
    let args = Args::parse();
    generate_color_data_and_encode(&colors, args.zoom);
}

fn generate_color_data_and_encode(colors: &[&[&str]], zoom: u16) {
    let mut data: Vec<u8> = Vec::new();
    for row in colors.iter() {
        for _ in 0..zoom {
            for col in row.iter() {
                for _ in 0..zoom {
                    if *col == "b" {
                        data.push(0);
                        data.push(0);
                        data.push(0);
                        data.push(255);
                    } else if *col == "w" {
                        data.push(255);
                        data.push(255);
                        data.push(255);
                        data.push(255);
                    }
                }
            }
        }
    }
    let height = colors.iter().count() as u32 * zoom as u32;

    let r0 = colors[0];
    let width = r0.len() as u32 * zoom as u32;
    
    encode_png(width, height, data);
}

fn encode_png(width: u32, height: u32, data: Vec<u8>) {
    let path = Path::new(r"image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    
    let mut encoder = png::Encoder::new(w, width, height);
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