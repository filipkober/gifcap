use image::{DynamicImage, Rgba, GenericImageView};
use rusttype::{Scale, point, Font};

use crate::gif::Color;

pub fn text_to_image(font: Font, text_to_render: &str, font_size: f32) {

    let scale = Scale::uniform(font_size);
    let color = (0, 0, 0);

    let padding = (font_size * 0.2).ceil() as u32;

    let v_metrics = font.v_metrics(scale);
    let glyphs: Vec<_> = font
        .layout(&text_to_render, scale, point(0.0, v_metrics.ascent))
        .collect();

    let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
    let glyphs_width = glyphs
        .iter()
        .rev()
        .map(|g| g.position().x as f32 + g.unpositioned().h_metrics().advance_width)
        .next()
        .unwrap_or(0.0)
        .ceil() as u32;

    let mut image = DynamicImage::new_rgba8(glyphs_width + padding, glyphs_height + padding).to_rgba8();

    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                image.put_pixel(
                    (padding / 2) + x + bounding_box.min.x as u32,
                    (padding / 2) + y + bounding_box.min.y as u32,
                    Rgba([color.0, color.1, color.2, (v * 255.0) as u8]),
                )
            });
        }
    }

    for x in 0..image.width() {
        for y in 0..image.height() {
            let pixel = image.get_pixel(x, y);
            if pixel[3] == 0 {
                image.put_pixel(x, y, Rgba([255, 255, 255, 255]));
            }
        }
    }

    image.save("output.png").unwrap();
}
pub fn image_to_lzw(image: &DynamicImage, colors: &Vec<Color>) -> Vec<u8> {
    let mut lzw = Vec::new();

    let mut color_map = Vec::new();
    for color in colors {
        color_map.push(color.red);
        color_map.push(color.green);
        color_map.push(color.blue);
    }

    let mut color_map = color_map.chunks(3);

    let mut current_byte = 0;
    let mut current_bit = 0;

    for pixel in image.pixels() {
        let pixel = pixel.2;

        let mut color_index = 0;
        for (i, color) in color_map.clone().enumerate() {
            if color[0] == pixel[0] && color[1] == pixel[1] && color[2] == pixel[2] {
                color_index = i;
                break;
            }
        }

        current_byte |= (color_index << current_bit) as u8;

        current_bit += 2;

        if current_bit == 8 {
            lzw.push(current_byte);
            current_byte = 0;
            current_bit = 0;
        }
    }

    lzw
}