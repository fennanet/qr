use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use png;
use clap::{Parser, ValueEnum};


#[derive(Parser, Debug)]
#[command(name = "qr", version, about("qr - (c) fenna"), long_about("qr - (c) fenna"))]
struct Args {
    ///how much to zoom in on each pixel
    #[arg(short, long, default_value = "100")]
    zoom: u16,

    ///contents of the QR code
    #[arg(short, long, default_value = "")]
    content: String,

    ///output file path
    #[arg(short, long, default_value = "qr.png")]
    output: String,

    /// redundancy level
    #[arg(short, long, default_value = "l")]
    redundancy: RedundancyLevel,
}

#[derive(ValueEnum, Debug, Clone)]
enum RedundancyLevel {
    L,
    M,
    Q,
    H,
}

fn main() {
    let colors: Vec<&[&u8]> = vec![
        &[&1, &0, &1, &0],
        &[&0, &1, &0, &1],
        &[&1, &0, &1, &0],
        &[&0, &1, &0, &1],
    ];
    let args = Args::parse();

    let bin = txt_to_bin(&args.content);
    generate_color_data_and_encode(&colors, args.zoom, &args.output);

    for char in bin.split(" "){
        println!("{}", char)
    }

    println!("{}", bin.split(" ").count() - 1);
    println!("{}", bin)
}

fn txt_to_bin(text: &str) -> String {
    let mut bin = "".to_string();
    
    for character in text.to_string().clone().into_bytes() {
        bin += &format!("0{:b} ", character);
    }
    bin
}



fn generate_color_data_and_encode(colors: &[&[&u8]], zoom: u16, output: &str) {
    let mut data: Vec<u8> = Vec::new();
    for row in colors.iter() {
        for _ in 0..zoom {
            for col in row.iter() {
                for _ in 0..zoom {
                    if **col == 1 {
                        data.push(0);
                        data.push(0);
                        data.push(0);
                        data.push(255);
                    } else if **col == 0 {
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
    
    encode_png(width, height, data, output);
}

fn encode_png(width: u32, height: u32, data: Vec<u8>, output: &str) {
    let path = Path::new(output);
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