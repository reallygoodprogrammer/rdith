use clap::Parser;
use image::{
    Pixel, Rgba, RgbaImage,
    imageops::{Lanczos3, resize},
    open,
};
use rand::Rng;

#[derive(Parser)]
struct Args {
    input_file: String,
    output_file: String,
    #[arg(long, default_value_t = String::from(""))]
    layer: String,
    #[arg(long, default_value_t = false)]
    predithlayer: bool,
}

const MSIZE: usize = 5;

fn main() {
    let args = Args::parse();

    let mut in_file = open(args.input_file)
        .expect("could not open input file")
        .to_rgba8();
    match args.layer.as_str() {
        "" => {
            in_file = black_dither(in_file, create_matrix());
        }
        _ => {
            let layer_file = open(args.layer)
                .expect("could not open layer file")
                .to_rgba8();
            let new_dims = in_file.dimensions();
            let mut layer_file = resize(&layer_file, new_dims.0, new_dims.1, Lanczos3);
            if args.predithlayer {
                layer_file = black_dither(layer_file, create_matrix());
            }
            in_file = layer_dither(in_file, layer_file, create_matrix());
        }
    }

    in_file
        .save(args.output_file)
        .expect("could not save output file");
}

fn create_matrix() -> [[f32; MSIZE]; MSIZE] {
    let mut matrix: [[f32; MSIZE]; MSIZE] = [[0.0; MSIZE]; MSIZE];
    let mut rng = rand::rng();
    for col in matrix.iter_mut() {
        rng.fill(col);
        col.iter_mut().for_each(|v| {
            *v *= 255.0;
        });
    }
    matrix
}

fn black_dither(mut image: RgbaImage, matrix: [[f32; MSIZE]; MSIZE]) -> RgbaImage {
    for (x, y, p) in image.enumerate_pixels_mut() {
        let mp = matrix[x as usize % MSIZE][y as usize % MSIZE];
        if mp > p.to_luma().0[0] as f32 {
            *p = Rgba([0u8, 0u8, 0u8, 255u8]);
        }
    }
    image
}

fn layer_dither(
    mut image: RgbaImage,
    layer: RgbaImage,
    matrix: [[f32; MSIZE]; MSIZE],
) -> RgbaImage {
    assert!(image.dimensions() == layer.dimensions());
    for (x, y, p) in image.enumerate_pixels_mut() {
        let mp = matrix[x as usize % MSIZE][y as usize % MSIZE];
        if mp > p.to_luma().0[0] as f32 {
            let lp = layer.get_pixel(x, y);
            *p = *lp;
        }
    }
    image
}
