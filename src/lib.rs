
extern crate svg;

pub fn render((width, height): (u64, u64), events: Vec<svg::SvgEvent>) -> Vec<u8> {
    let mut canvas = Vec::new();

    for _ in 0 .. width * height {
        canvas.extend_from_slice(&[255, 255, 255, 0])
    }

    for svg::SvgEvent::Line { x1, x2, y1, y2, view_box, stroke, stroke_width } in events {
        let (re_x1, re_x2, re_y1, re_y2);
        let avg;
        {
            let width = view_box[2] - view_box[0];
            let height = view_box[3] - view_box[1];
            re_x1 = (x1 - view_box[0]) / width;
            re_x2 = (x2 - view_box[0]) / width;
            re_y1 = (y1 - view_box[1]) / height;
            re_y2 = (y2 - view_box[1]) / height;
            avg = f64::sqrt(width * height);
        }

        for offset in 0 .. width * height {
            let x = offset % height;
            let y = offset / height;

            let re_x = x as f64 / width as f64;
            let re_y = y as f64 / height as f64;

            use distance::hit;

            if hit(stroke_width / avg / 2.0, (re_x1, re_y1, re_x2, re_y2), re_x, re_y) {
                canvas[4 * offset as usize + 0] = stroke.red;
                canvas[4 * offset as usize + 1] = stroke.green;
                canvas[4 * offset as usize + 2] = stroke.blue;
				canvas[4 * offset as usize + 3] = 255
            }
        }
    }

    canvas
}

mod distance;
