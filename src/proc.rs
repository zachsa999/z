use crate::{create_transform, types::ImagePixels};
use image::ImageError;
use image::{io::Reader as ImageReader, DynamicImage, GenericImageView, ImageBuffer, Rgba};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::result::Result;

pub fn delete_rows_with_fsc_or_fuel(
    input_file: &str,
    output_file: &str,
) -> Result<(), Box<dyn Error>> {
    let input = File::open(input_file)?;
    let output = File::create(output_file)?;
    let mut writer = BufWriter::new(output);

    for line in BufReader::new(input).lines() {
        let line = line?;
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() >= 3
            && (fields[2].contains("fsc")
                || fields[2].contains("FSC")
                || fields[2].contains("fuel")
                || fields[2].contains("Fuel"))
        {
            continue;
        }
        writeln!(writer, "{}", line)?;
    }

    Ok(())
}

pub fn transform_image(file_name: &str, output_name: &str) -> Result<(), ImageError> {
    let (prepare_image, width, height) = read_image_from_file(file_name).unwrap();
    let transformed: DynamicImage = create_image_from_pixels(&prepare_image, width, height);
    let result = transformed.save(output_name);
    result
}

fn read_image_from_file(file_name: &str) -> Result<(ImagePixels, u32, u32), Box<dyn Error>> {
    let dynamic_image: DynamicImage = ImageReader::open(file_name)?.decode()?;
    let (width, height) = dynamic_image.dimensions();

    let pixels: ImagePixels = convert_image_to_pixels(&dynamic_image);
    Ok((pixels, width, height))
}

fn convert_image_to_pixels(img: &DynamicImage) -> ImagePixels {
    let (width, height) = img.dimensions();
    let mut pixels: ImagePixels = vec![vec![0u8; 4]; width as usize * height as usize];

    for (x, y, pixel) in img.pixels() {
        let index = (x + y * width) as usize;
        pixels[index][0] = pixel[0];
        pixels[index][1] = pixel[1];
        pixels[index][2] = pixel[2];
        pixels[index][3] = pixel[3];
    }

    overlay_pixels(&pixels)
}

fn overlay_pixels(pixels: &ImagePixels) -> ImagePixels {
    let mut new_pixels: ImagePixels = vec![vec![0u8; 4]; pixels.len()];
    let white_pixels: ImagePixels = vec![vec![255u8; 4]; pixels.len()];

    for i in 0..pixels.len() {
        let r1 = pixels[i][0] as f32 / 255.0;
        let g1 = pixels[i][1] as f32 / 255.0;
        let b1 = pixels[i][2] as f32 / 255.0;
        let a1 = pixels[i][3] as f32 / 255.0;

        let r2 = white_pixels[i][0] as f32 / 255.0;
        let g2 = white_pixels[i][1] as f32 / 255.0;
        let b2 = white_pixels[i][2] as f32 / 255.0;
        let a2 = white_pixels[i][3] as f32 / 255.0;

        let r: f32;
        let g: f32;
        let b: f32;
        let a: f32;

        if a2 == 0.0 {
            r = r1;
            g = g1;
            b = b1;
            a = a1;
        } else {
            r = (1.0 - a2) * r1 + a2 * r2;
            g = (1.0 - a2) * g1 + a2 * g2;
            b = (1.0 - a2) * b1 + a2 * b2;
            a = a1 + a2 * (1.0 - a1);
        }

        new_pixels[i][0] = (r * 255.0) as u8;
        new_pixels[i][1] = (g * 255.0) as u8;
        new_pixels[i][2] = (b * 255.0) as u8;
        new_pixels[i][3] = (a * 255.0) as u8;
    }

    apply_transform(new_pixels)
}

fn apply_transform(pixels: ImagePixels) -> ImagePixels {
    let transformed: ImagePixels = create_transform(&pixels);
    transformed
}

fn create_image_from_pixels(pixels: &ImagePixels, width: u32, height: u32) -> DynamicImage {
    let mut image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
        let index = (x + y * width) as usize;
        let rgba = Rgba([
            pixels[index][0],
            pixels[index][1],
            pixels[index][2],
            pixels[index][3],
        ]);
        *pixel = rgba;
    }

    DynamicImage::ImageRgba8(image_buffer)
}
