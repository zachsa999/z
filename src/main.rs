mod proc;
mod types;
use proc::{delete_rows_with_fsc_or_fuel, transform_image};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use types::ImagePixels;

fn main() {
    println!("Starting");
    // let img_name: &str = "src/small_z.png";
    // let out_name: &str = "new_z.png";
    // let new_img = transform_image(img_name, out_name);
    // println!("New image: {:?}", new_img);
    let _ = delete_rows_with_fsc_or_fuel("src/f4f_loads.csv", "out.csv");
}

pub fn create_transform(pixels: &ImagePixels) -> ImagePixels {
    pixels.clone()
}
