//! # rdith
//!
//! tool for manipulating images through dithering operations.

use clap::Parser;
use image::{
    Pixel, Rgba, RgbaImage,
    imageops::{Lanczos3, resize},
    open,
};
use rand::Rng;

#[derive(Parser)]
struct Args {
    #[arg(help = "input files for dithering")]
    input_file: String,
    #[arg(long, default_value_t = String::from("output.png"), help = "name of output file")]
    output_file: String,
    #[arg(long, help = "background layer file")]
    layer: Option<String>,
    #[arg(long, default_value_t = false, help = "predither the background layer")]
    predith: bool,
}

fn main() {
    let args = Args::parse();

    let mut in_file = open(args.input_file)
        .expect("could not open input file")
        .to_rgba8();
    match args.layer {
        None => {
            in_file = black_dither(in_file, create_matrix(5));
        }
        Some(layer_file_path) => {
            let layer_file = open(layer_file_path)
                .expect("could not open layer file")
                .to_rgba8();
            let new_dims = in_file.dimensions();
            let mut layer_file = resize(&layer_file, new_dims.0, new_dims.1, Lanczos3);
            if args.predith {
                layer_file = black_dither(layer_file, create_matrix(5));
            }
            in_file = layer_dither(in_file, layer_file, create_matrix(5));
        }
    }

    in_file
        .save(args.output_file)
        .expect("could not save output file");
}

/// Create a new f32 matrix with dimensions `msize`X`msize`.
fn create_matrix(msize: usize) -> Vec<Vec<f32>> {
    let mut v: Vec<Vec<f32>> = Vec::new();
    let mut rng = rand::rng();
    for _ in 0..msize {
        let mut row: Vec<f32> = Vec::new();
        for _ in 0..msize {
            row.push(255.0 * rng.random::<f32>());
        }
        v.push(row);
    }
    v
}

/// Dither `image` using the dithering matrix `matrix`.
fn black_dither(mut image: RgbaImage, matrix: Vec<Vec<f32>>) -> RgbaImage {
    let msize = matrix.len();
    for (x, y, p) in image.enumerate_pixels_mut() {
        let mp = matrix[x as usize % msize][y as usize % msize];
        if mp > p.to_luma().0[0] as f32 {
            *p = Rgba([0u8, 0u8, 0u8, 255u8]);
        }
    }
    image
}

/// Dither `image` using the `layer` image as the background layer
/// with dithering matrix `matrix`.
fn layer_dither(mut image: RgbaImage, layer: RgbaImage, matrix: Vec<Vec<f32>>) -> RgbaImage {
    assert!(image.dimensions() == layer.dimensions());
    let msize = matrix.len();
    for (x, y, p) in image.enumerate_pixels_mut() {
        let mp = matrix[x as usize % msize][y as usize % msize];
        if mp > p.to_luma().0[0] as f32 {
            let lp = layer.get_pixel(x, y);
            *p = *lp;
        }
    }
    image
}
