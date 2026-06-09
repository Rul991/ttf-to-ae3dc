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
    let mut y = 0usize;

    let mut max_line_height = 0usize;
    let mut line_height = 0usize;
    let mut i = 0usize;

    let glyph_map = font.chars();

    let chars: Vec<char> = if args.draw_symbols.is_empty() {
        glyph_map.keys().copied().collect()
    } else {
        args.draw_symbols.chars().collect()
    };

    let chars_count = chars.len();
    let chars_per_line = chars_count.isqrt() + 1;

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
        line_height = height.max(line_height);

        let advance = advance_width + letter_spacing;
        let width = if width != 0 {
            width
        } else {
            advance as usize
        };

        let glyph = Glyph {
            id: symbol as u16,
            x,
            y,
            w: width,
            h: height,
            ox: xmin,
            oy: font_size as i32 - (height as i32 + ymin),
            advance,
        };

        let mut data = Vec::with_capacity(bitmap.len() * 4);
        for &alpha in &bitmap {
            data.extend_from_slice(&[255, 255, 255, alpha]);
        }

        let letter_image = ImageBuffer::<Rgba<u8>, _>::from_raw(width as u32, height as u32, data)
            .expect("Failed to create image for letter");

        images.push(letter_image);
        glyphs.push(glyph);

        i += 1;
        
        if (i % chars_per_line) == 0 {
            x = 0;
            y += line_height;

            max_line_height = max_line_height.max(line_height);
            line_height = 0;
        }
        else {
            x += width;
        }
    }
    max_line_height = max_line_height.max(line_height);

    println!("Glyphs processed: {}", images.len());

    let max_bottom = glyphs
        .iter()
        .max_by(|a, b| {
            let a_bottom = a.bottom();
            let b_bottom = b.bottom();

            a_bottom.cmp(&b_bottom)
        })
        .expect("Failed to get maximum height of glyphs")
        .bottom();

    let max_right = glyphs
        .iter()
        .max_by(|a, b| {
            let a_right = a.right();
            let b_right = b.right();

            a_right.cmp(&b_right)
        })
        .expect("Failed to get maximum height of glyphs")
        .right();

    let mut image = ImageBuffer::from_pixel(
        max_right.try_into().unwrap(),
        max_bottom.try_into().unwrap(),
        Rgba([0u8; 4]),
    );

    for i in 0..glyphs.len() {
        let Glyph { x, y, .. } = glyphs.get(i).unwrap();
        let letter_image = images.get(i).unwrap().clone();

        overlay(&mut image, &letter_image, (*x) as i64, (*y) as i64);
    }

    println!("Texture atlas created ({}x{})", max_right, max_bottom);

    let mut json_glyphs: json::JsonValue = array![];

    for glyph in &glyphs {
        let _ = json_glyphs.push(glyph.to_json(show_symbol));
    }

    let json_result = object! {
        lineHeight: max_line_height,
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
