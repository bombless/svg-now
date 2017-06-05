extern crate rusttype;

use self::rusttype::{FontCollection, Scale, point, PositionedGlyph};

pub fn render((canvas_width, canvas_height): (u64, u64), content: &str, canvas: &mut [u8]) {
    let font_data = include_bytes!("../emoji.ttf");
    let collection = FontCollection::from_bytes(&font_data[..]);
    let font = collection.into_font().unwrap(); // only succeeds if collection consists of one font

    // Desired font pixel height
    let height: f32 = (canvas_height / 2) as _; // to get 80 chars across (fits most terminals); adjust as desired
    let _pixel_height = height.ceil() as usize;

    // 2x scale in x direction to counter the aspect ratio of monospace characters.
    let scale = Scale { x: height*2.0, y: height };

    // The origin of a line of text is at the baseline (roughly where non-descending letters sit).
    // We don't want to clip the text, so we shift it down with an offset when laying it out.
    // v_metrics.ascent is the distance between the baseline and the highest edge of any glyph in
    // the font. That's enough to guarantee that there's no clipping.
    let v_metrics = font.v_metrics(scale);
    let offset = point(0.0, v_metrics.ascent);

    // Glyphs to draw for "RustType". Feel free to try other strings.
    let glyphs: Vec<PositionedGlyph> = font.layout(content, scale, offset).collect();

    // Find the most visually pleasing width to display
    let _width = glyphs.iter().rev()
        .filter_map(|g| g.pixel_bounding_box()
                    .map(|b| b.min.x as f32 + g.unpositioned().h_metrics().advance_width))
        .next().unwrap_or(0.0).ceil() as usize;

    for g in glyphs {
        if let Some(bb) = g.pixel_bounding_box() {
            g.draw(|x, y, v| {
                let x = x as i32 + bb.min.x;
                let y = y as i32 + bb.min.y;
                // There's still a possibility that the glyph clips the boundaries of the bitmap
                if x >= 0 && x < canvas_width as i32 && y >= 0 && y < canvas_height as i32 {
                    let x = x as usize;
                    let y = y as usize;
                    let p = 4 * (x + y * canvas_width as usize);
                    let gray_scale = (v * 256.0) as u8;
                    canvas[p] = gray_scale;
                    canvas[p + 1] = gray_scale;
                    canvas[p + 2] = gray_scale;
                    canvas[p + 3] = 255;
                }
            })
        }
    }
 }
