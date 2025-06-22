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
    input_files: Vec<String>,
    #[arg(long, default_value_t = String::from("output.png"), help = "name of output file")]
    output_file: String,
    #[arg(long, default_value_t = false, help = "predither the background layer")]
    predith: bool,
    #[arg(
        short,
        long,
        default_value_t = 5,
        help = "dimensions of dithering matrix"
    )]
    dims_matrix: usize,
    #[arg(
        short,
        long,
        default_value_t = 1,
        help = "resolution of dithering matrix"
    )]
    res_matrix: usize,
}

fn main() {
    let args = Args::parse();

    let mut in_file = open(args.input_files[0].clone())
        .expect("could not open input file")
        .to_rgba8();
    if args.predith {
        in_file = black_dither(in_file, create_matrix(args.dims_matrix, args.res_matrix));
    }

    for file in &args.input_files[1..] {
        let layer_file = open(file).expect("could not open layer file").to_rgba8();
        let new_dims = in_file.dimensions();
        let layer_file = resize(&layer_file, new_dims.0, new_dims.1, Lanczos3);
        in_file = layer_dither(
            in_file,
            layer_file,
            create_matrix(args.dims_matrix, args.res_matrix),
        );
    }

    in_file
        .save(args.output_file)
        .expect("could not save output file");
}

/// Create a new f32 matrix with dimensions `msize`X`msize`.
fn create_matrix(msize: usize, mres: usize) -> Vec<Vec<f32>> {
    let mut v: Vec<Vec<f32>> = Vec::new();
    let mut rng = rand::rng();
    for _ in 0..msize {
        let mut row: Vec<f32> = Vec::new();
        for _ in 0..msize {
            let val = 255.0 * rng.random::<f32>();
            for _ in 0..(mres - 1) {
                row.push(val);
            }
            row.push(val);
        }
        for _ in 0..(mres - 1) {
            v.push(row.clone());
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
