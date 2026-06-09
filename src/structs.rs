use clap::Parser;
use json::{JsonValue, object};

/// Convert ttf to format for ae3dc engine (png + json)
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CommandArgs {
    /// Font name (if path is "font.ttf", then name will be "font")
    #[arg(short, long)]
    pub name: String,

    /// Font height in pixels
    #[arg(short, long, default_value_t = 64)]
    pub size: u8,

    /// Amount of empty space between characters
    #[arg(short, long, default_value_t = 2)]
    pub letter_spacing: u16,

    /// Specifies which characters should be drawn on the image, by default all characters
    #[arg(short, long, default_value_t = String::new())]
    pub draw_symbols: String,

    /// Shows a symbol in a JSON file
    #[arg(long, default_value_t = false)]
    pub show_symbol: bool,
}

#[derive(Debug, Clone)]
pub struct Glyph {
    pub id: u16,
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
    pub ox: i32,
    pub oy: i32,
    pub advance: f32,
}

impl Glyph {
    pub fn to_json(&self, show_symbol: bool) -> JsonValue {
        if show_symbol {
            let symb = char::from_u32(self.id as u32).unwrap().to_string();
            object! {
                id: self.id,
                x: self.x,
                y: self.y,
                w: self.w,
                h: self.h,
                ox: self.ox,
                oy: self.oy,
                advance: self.advance.ceil(),
                symb: symb
            }
        }
        else {
            object! {
                id: self.id,
                x: self.x,
                y: self.y,
                w: self.w,
                h: self.h,
                ox: self.ox,
                oy: self.oy,
                advance: self.advance.ceil(),
            }
        }
    }

    pub fn bottom(&self) -> usize {
        self.y + self.h
    }

    pub fn right(&self) -> usize {
        self.x + self.w
    }
}

pub struct Paths {
    pub input_path: String,
    pub output_json_path: String,
    pub output_png_path: String,
}

impl Paths {
    pub fn new(name: String) -> Self {
        Self {
            input_path: format!("{}.ttf", name),
            output_json_path: format!("{}.json", name),
            output_png_path: format!("{}.png", name),
        }
    }
}
