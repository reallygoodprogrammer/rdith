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
    input_file: String,
    #[arg(long, default_value_t = String::from("output.png"))]
    output_file: String,
    #[arg(long, default_value_t = String::from(""))]
    layer: String,
    #[arg(long, default_value_t = false)]
    predith: bool,
}

fn main() {
    let args = Args::parse();

    let mut in_file = open(args.input_file)
        .expect("could not open input file")
        .to_rgba8();
    match args.layer.as_str() {
        "" => {
            in_file = black_dither(in_file, create_matrix(5));
        }
        _ => {
            let layer_file = open(args.layer)
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
