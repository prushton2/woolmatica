use std::io::Cursor;
use image::{RgbImage, Rgb, ImageReader, GenericImageView, ImageFormat};
use std::ops::IndexMut;

fn divide_pixel(r: u8, g: u8, b: u8, dividend: u64) -> Rgb<u8> {
    let mut new_rgb = Rgb::<u8>([
        ((r as u64) / dividend).try_into().unwrap(),
        ((g as u64) / dividend).try_into().unwrap(),
        ((b as u64) / dividend).try_into().unwrap(),
    ]);

    return new_rgb;
}

fn main() {

    let scaling: u32 = 2;
    let scale_sq: u64 = scaling.pow(2).into();

    let img_in = image::open("prog_output.png").unwrap();
    
 //   let img_out = ImageReader::new(Cursor::new("")).set_format(ImageFormat::Png);
    let (w, h) = img_in.dimensions();

    let mut img_out = RgbImage::new(w,h);

    for pixel in img_in.pixels() {
        img_out.put_pixel(pixel.0/scaling, pixel.1/scaling, divide_pixel(pixel.2[0], pixel.2[1], pixel.2[2], scale_sq));
    }

    img_out.save_with_format("./out.png", ImageFormat::Png);
    
}
