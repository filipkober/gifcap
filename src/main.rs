#![allow(unused_imports, dead_code)]
use std::env;
use gif::GIF;
use rusttype::Font;
use image::{Rgb, RgbImage};

mod text_to_image;

mod gif;

#[allow(unused_variables)]
fn main() {
    // let args: Vec<String> = env::args().collect();

    // let font_file_path = args.get(1).expect("Missing argument: font file path");

    // let text_to_render = args.get(2).expect("Missing argument: text to render");

    // let font_size = if let Some(arg) = args.get(3) {
    //     arg.parse::<f32>().expect("Unable to parse font size")
    // } else {
    //     128f32
    // };

    // let font_data: Vec<u8> = std::fs::read(font_file_path).expect("Unable to read font file");
    // let font = Font::try_from_bytes(&font_data).expect("Unable to create font from bytes");

    // text_to_image::text_to_image(font, text_to_render, font_size);

    let gif = GIF::from_file("loading.gif");
    gif.reverse().save("loading-reversed.gif");
}
