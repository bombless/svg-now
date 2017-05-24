
extern crate svg;

pub fn render((width, height): (u64, u64), events: Vec<svg::SvgEvent>) -> Vec<u8> {
    let mut canvas = Vec::new();

    for _ in 0 .. width * height {
        canvas.extend_from_slice(&[255, 255, 255, 0])
    }

    for svg::SvgEvent::Line { x1, x2, y1, y2, view_box } in events {
        let (re_x1, re_x2, re_y1, re_y2);
        {
            let width = view_box[2] - view_box[0];
            let height = view_box[3] - view_box[1];
            re_x1 = (x1 - view_box[0]) / width;
            re_x2 = (x2 - view_box[0]) / width;
            re_y1 = (y1 - view_box[1]) / height;
            re_y2 = (y2 - view_box[1]) / height;
        }

        for offset in 0 .. width * height {
            let x = offset % height;
            let y = offset / height;

            let re_x = x as f64 / width as f64;
            let re_y = y as f64 / height as f64;

            use distance::hit;

            if hit((re_x1, re_y1, re_x2, re_y2), re_x, re_y) {
                for i in 0 .. 3 {
                    canvas[4 * offset as usize + i] = 0
                }
				canvas[4 * offset as usize + 3] = 255
            }
        }
    }

    canvas
}

mod distance;
