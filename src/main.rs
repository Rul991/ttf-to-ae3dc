use clap::Parser;
use fontdue::{Font, FontSettings, Metrics};
use image::{ImageBuffer, Rgba, imageops::overlay};
use json::{array, object};
use std::fs::{read, write};

use crate::structs::{CommandArgs, Glyph, Paths};

mod structs;

fn read_font(path: &str) -> Font {
    let font_data = read(path).expect("Failed to read input file");

    let font = Font::from_bytes(font_data, FontSettings::default()).expect("Failed to parse font");

    font
}

fn main() {
    let args = CommandArgs::parse();

    let Paths {
        input_path,
        output_json_path,
        output_png_path,
    } = Paths::new(args.name);

    let font = read_font(&input_path);
    println!("Font loaded from: {input_path}");

    let letter_spacing = args.letter_spacing as f32;
    let font_size = args.size as f32;
    let show_symbol = args.show_symbol;

    let mut images = vec![];
    let mut glyphs: Vec<Glyph> = vec![];
    let mut x = 0usize;

    let glyph_map = font.chars();

    let chars: Vec<char> = if args.draw_symbols.is_empty() {
        glyph_map.keys().copied().collect()
    } else {
        args.draw_symbols.chars().collect()
    };

    for symbol in chars {
        let (metrics, bitmap) = font.rasterize(symbol, font_size);

        let Metrics {
            xmin,
            ymin,
            width,
            height,
            advance_width,
            ..
        } = metrics;

        let glyph = Glyph {
            id: symbol as u16,
            x,
            y: 0,
            w: width,
            h: height,
            ox: xmin,
            oy: ymin,
            advance: advance_width + letter_spacing,
        };

        let mut data = Vec::with_capacity(bitmap.len() * 4);
        for &alpha in &bitmap {
            data.extend_from_slice(&[255, 255, 255, alpha]);
        }

        let letter_image = ImageBuffer::<Rgba<u8>, _>::from_raw(width as u32, height as u32, data)
            .expect("Failed to create image for letter");

        images.push(letter_image);
        glyphs.push(glyph);
        x += width;
    }

    println!("Glyphs processed: {}", images.len());

    let max_glyph = glyphs
        .iter()
        .max_by(|a, b| a.h.cmp(&b.h))
        .expect("Failed to get maximum height of glyphs");

    let max_height = max_glyph.h;
    let max_width = x;
    let mut image = ImageBuffer::from_pixel(
        max_width.try_into().unwrap(),
        max_height.try_into().unwrap(),
        Rgba([0u8; 4]),
    );

    for i in 0..glyphs.len() {
        let Glyph { x, y, .. } = glyphs.get(i).unwrap();
        let letter_image = images.get(i).unwrap().clone();

        overlay(&mut image, &letter_image, (*x) as i64, (*y) as i64);
    }

    println!("Texture atlas created ({}x{})", max_width, max_height);

    let mut json_glyphs: json::JsonValue = array![];

    for glyph in &glyphs {
        let _ = json_glyphs.push(glyph.to_json(show_symbol));
    }

    let json_result = object! {
        lineHeight: max_height,
        glyphs: json_glyphs
    };

    if let Err(e) = write(&output_json_path, json_result.to_string()) {
        eprintln!("Error while saving json: {e:?}");
        return;
    }

    println!("JSON saved to: {}", &output_json_path);

    if let Err(e) = image.save(&output_png_path) {
        eprintln!("Error while saving png: {e:?}");
        return;
    }

    println!("PNG saved to: {}", &output_png_path);
}
