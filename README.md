# Font to Atlas Converter for ae3dc Engine

This tool converts a TrueType font (`.ttf`) into a texture atlas (PNG) and a JSON metadata file suitable for the **Ae3Dc** engine. It rasterizes characters at a specified size, computes glyph metrics, and arranges them horizontally in a single row.

## Features

- Renders all supported glyphs or a user‑specified subset.
- Configurable font size and letter spacing.
- Outputs:
  - A **PNG** atlas (single row, height = tallest glyph).
  - A **JSON** file containing glyph positions, sizes, offsets, and advance widths.
- Pure Rust implementation using `fontdue` for font rasterisation and `image` for texture creation.

## Usage

```bash
ttf-to-ae3dc --name <FONT_NAME> [--size <SIZE>] [--letter-spacing <SPACING>] [--draw-symbols <CHARS>]
```

### Arguments

| Option               | Description                                                                 | Default          |
|----------------------|-----------------------------------------------------------------------------|------------------|
| `-n, --name <NAME>`  | Font name. The tool expects `<NAME>.ttf` as input and produces `<NAME>.json` and `<NAME>.png`. | **required** |
| `-s, --size <SIZE>`  | Font height in pixels (rasterisation size).                                 | `64`             |
| `-l, --letter-spacing <SPACING>` | Extra empty space (in pixels) added between characters.                    | `2`              |
| `-d, --draw-symbols <CHARS>` | String specifying exactly which characters to render. If empty, all glyphs supported by the font are rendered. | (empty) |

This reads `Roboto.ttf`, rasterises the characters `A`, `B`, `C`, `1`, `2`, `3` at 48 px height with 1 px extra spacing, and writes:

- `Roboto.png` – the texture atlas.
- `Roboto.json` – the glyph metadata.

## Output Format

### PNG Atlas

- **Width** = sum of all rasterised glyph widths + total letter spacing.
- **Height** = maximum glyph height among the rendered characters.
- Pixels are stored as **LumaA** (grayscale + alpha). The texture can be used directly for GPU text rendering.

### JSON Metadata

```json
{
  "lineHeight": <max_glyph_height_in_pixels>,
  "glyphs": [
    {
      "id": <unicode_codepoint>,
      "x": <left_position_in_atlas>,
      "y": <top_position_in_atlas>,
      "w": <glyph_width>,
      "h": <glyph_height>,
      "ox": <horizontal_offset_from_cursor>,
      "oy": <vertical_offset_from_baseline>,
      "advance": <advance_width_ceiled_to_next_integer>
    },
    ...
  ]
}
```

**Field meanings:**

- `id` – Unicode code point of the character.
- `x`, `y` – top‑left corner of the glyph in the atlas.
- `w`, `h` – width and height of the glyph’s bounding box.
- `ox`, `oy` – offset from the current pen position to where the glyph should be drawn (typically used for kerning and baseline alignment).
- `advance` – distance (in pixels) to advance the pen after drawing this glyph.

## Building

The project uses Cargo. Make sure you have Rust installed, then:

```bash
cargo build --release
```

The executable will be located in `target/release/`.

## Notes

- The tool arranges glyphs in a **single horizontal row**. For fonts with many glyphs, the resulting PNG can become very wide.
- `advance` is ceiled to an integer to simplify rendering loops in the engine.