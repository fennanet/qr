use std::{path::Path};
use std::fs::File;
use std::io::BufWriter;
use png;
use clap::{Parser, ValueEnum};

pub const ALIGNMENT_PATTERN_COORDS: &[&[u8]] = &[
    &[6, 18],
    &[6, 22],
    &[6, 26],
    &[6, 30],
    &[6, 34],
    &[6, 22, 38],
    &[6, 24, 42],
    &[6, 26, 46],
    &[6, 28, 50],
    &[6, 30, 54],
    &[6, 32, 58],
    &[6, 34, 62],
    &[6, 26, 46, 66],
    &[6, 26, 48, 70],
    &[6, 26, 50, 74],
    &[6, 30, 54, 78],
    &[6, 30, 56, 82],
    &[6, 30, 58, 86],
    &[6, 34, 62, 90],
    &[6, 28, 50, 72, 94],
    &[6, 26, 50, 74, 98],
    &[6, 30, 54, 78, 102],
    &[6, 28, 54, 80, 106],
    &[6, 32, 58, 84, 110],
    &[6, 30, 58, 86, 114],
    &[6, 34, 62, 90, 118],
    &[6, 26, 50, 74, 98, 122],
    &[6, 30, 54, 78, 102, 126],
    &[6, 26, 52, 78, 104, 130],
    &[6, 30, 56, 82, 108, 134],
    &[6, 34, 60, 86, 112, 138],
    &[6, 30, 58, 86, 114, 142],
    &[6, 34, 62, 90, 118, 146],
    &[6, 30, 54, 78, 102, 126, 150],
    &[6, 24, 50, 76, 102, 128, 154],
    &[6, 28, 54, 80, 106, 132, 158],
    &[6, 32, 58, 84, 110, 136, 162],
    &[6, 26, 54, 82, 110, 138, 166],
    &[6, 30, 58, 86, 114, 142, 170],
];

pub const QR_CAPACITY: [[u16; 4]; 40] = [
    // L,    M,    Q,    H
    [17, 14, 11, 7],       // 1
    [32, 26, 20, 14],      // 2
    [53, 42, 32, 24],      // 3
    [78, 62, 46, 34],      // 4
    [106, 84, 60, 44],     // 5
    [134, 106, 74, 58],    // 6
    [154, 122, 86, 64],    // 7
    [192, 152, 108, 84],   // 8
    [230, 180, 130, 98],   // 9
    [271, 213, 151, 119],  // 10
    [321, 251, 177, 137],  // 11
    [367, 287, 203, 155],  // 12
    [425, 331, 241, 177],  // 13
    [458, 362, 258, 194],  // 14
    [520, 412, 292, 220],  // 15
    [586, 450, 322, 250],  // 16
    [644, 504, 364, 280],  // 17
    [718, 560, 394, 310],  // 18
    [792, 624, 442, 338],  // 19
    [858, 666, 482, 382],  // 20
    [929, 711, 509, 403],  // 21
    [1003, 779, 565, 439], // 22
    [1091, 857, 611, 461], // 23
    [1171, 911, 661, 511], // 24
    [1273, 997, 715, 535], // 25
    [1367, 1059, 751, 593],// 26
    [1465, 1125, 805, 625],// 27
    [1528, 1190, 868, 658],// 28
    [1628, 1264, 908, 698],// 29
    [1732, 1370, 982, 742],// 30
    [1840, 1452, 1030, 790],// 31
    [1952, 1538, 1112, 842],// 32
    [2068, 1628, 1168, 898],// 33
    [2188, 1722, 1228, 958],// 34
    [2303, 1809, 1283, 983],// 35
    [2431, 1911, 1351, 1051],// 36
    [2563, 1989, 1423, 1093],// 37
    [2699, 2099, 1499, 1139],// 38
    [2809, 2213, 1579, 1219],// 39
    [2953, 2331, 1663, 1273],// 40
];

pub const FINDER_PATTERN: [[u8; 7]; 7] = [
    [1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 1],
    [1, 0, 1, 1, 1, 0, 1],
    [1, 0, 1, 1, 1, 0, 1],
    [1, 0, 1, 1, 1, 0, 1],
    [1, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1]
];
pub const ALIGNMENT_PATTERN: [[u8; 5]; 5] = [
    [1, 1, 1, 1, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 1, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 1, 1, 1, 1]
];



#[derive(Parser, Debug)]
#[command(name = "qr", version, about("qr - (c) fenna"), long_about("qr - (c) fenna"))]
struct Args {
    ///how much to zoom in on each pixel
    #[arg(short, long, default_value = "100")]
    zoom: u16,

    ///contents of the QR code
    #[arg(short, long)]
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
    let mut colors: Vec<Vec<u8>> = vec![
        vec![1, 0, 1, 0],
        vec![0, 1, 0, 1],
        vec![1, 0, 1, 0],
        vec![0, 1, 0, 1],
    ];
    
    let args = Args::parse();

    let redundancy_level = match args.redundancy {
        RedundancyLevel::L => 0,
        RedundancyLevel::M => 1,
        RedundancyLevel::Q => 2,
        RedundancyLevel::H => 3,
    };

    let bin = txt_to_bin(&args.content);
    let version = decide_qr_version(&bin, redundancy_level);
    if version == 41 {
        if redundancy_level == 0 {
            println!("woahh, that's a lot of bytes! i'm afraid that it won't fit on to a single qr code")
        }
        else {
            println!("woahh, that's a lot of bytes! a lower amount of redundancy might work, other wise i'm afraid it won't fit on to a single qr code")
        }
    }
    else {
        colors = generate_qr(version, redundancy_level, &args.content);
        generate_color_data_and_encode(&colors, args.zoom, &args.output);
        print_to_terminal(&colors.clone());
        println!("qr code saved as image to {}", args.output)

    }

    
}

fn txt_to_bin(text: &str) -> String {
    let mut bin = "".to_string();
    
    for character in text.to_string().clone().into_bytes() {
        bin += &format!("0{:b} ", character);
    }
    bin
}

fn decide_qr_version(bin: &str, redundancy_level: u8) -> u8 {
    let bytesize = bin.split(" ").count() - 1;
    for version in 0..QR_CAPACITY.len() {
        if bytesize as u16 <= QR_CAPACITY[version][redundancy_level as usize] {
            return version as u8;
        }
    }
    41
}

fn generate_qr(version: u8, _redundancy_level: u8, _content: &str) -> Vec<Vec<u8>> {

    let qr_size = (version + 1) * 4 + 17;
    let mut qr: Vec<Vec<u8>> = Vec::new();
    for _ in 0..qr_size {
        qr.push(Vec::new());
    }
    for row in qr.iter_mut() {
        for _ in 0..qr_size {
            row.push(0);
        }
    }

    //qr emblems
    for (row_index, row) in FINDER_PATTERN.iter().enumerate() {
        for (cell_index, cell) in row.iter().enumerate() {
            qr[row_index][cell_index] = *cell;
            qr[qr_size as usize - 7 as usize + row_index][cell_index] = *cell;
            qr[row_index][qr_size as usize - 7 as usize + cell_index] = *cell;

        }
    }


    // timing strips
    for i in 0..qr_size-16{
        if i % 2 == 0 {
            qr[8+i as usize][6] = 1;
            qr[6][8+i as usize] = 1;
        }
    }

    // alignment patterns -> i add them after the timing strips because they override them :D
    if version >= 2 {
        for x in ALIGNMENT_PATTERN_COORDS[version as usize - 2] {
            for y in ALIGNMENT_PATTERN_COORDS[version as usize - 2] {
                let center_x = x + 2;
                let center_y = y +2;
                if !((center_x <= 9 && center_y <= 9) || (center_x <= 9 && center_y >= qr_size-9) || (center_x >= qr_size-9 && center_y <= 9)){
                    for (row_index, row) in ALIGNMENT_PATTERN.iter().enumerate() {
                        for (cell_index, cell) in row.iter().enumerate(){
                            qr[*y as usize + row_index][*x as usize + cell_index] = *cell;
                        }
                    }
                }
            }
        }
    }

    // black square
    qr[qr_size as usize - 8][8] = 1;

    
    
    
    qr
}

fn print_to_terminal(colors: &Vec<Vec<u8>>) {
    let mut print_row: String = "".to_string();
    for row in colors.iter() {
        for char in row {
            if *char == 0 {
                print_row += "  ";
            }
            else if *char == 1 {
                print_row += "██"
            }
            else {
                eprintln!("nasty character found!");
            }
            
        }
        println!("{}", print_row);
        print_row = "".to_string();
    }
}


fn generate_color_data_and_encode(colors: &Vec<Vec<u8>>, zoom: u16, output: &str) {
    let mut data: Vec<u8> = Vec::new();
    for row in colors.iter() {
        for _ in 0..zoom {
            for col in row.iter() {
                for _ in 0..zoom {
                    if *col == 1 {
                        data.push(0);
                        data.push(0);
                        data.push(0);
                        data.push(255);
                    } else if *col == 0 {
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

    let r0 = colors[0].clone();
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