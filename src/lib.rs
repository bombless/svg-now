
extern crate svg;

pub fn render((width, height): (u64, u64), svg::SvgEvents { events, view_box }: svg::SvgEvents) -> Vec<u8> {
    let mut canvas = Vec::new();

    for _ in 0 .. width * height {
        canvas.extend_from_slice(&[255, 255, 255, 0])
    }

    for e in events {
        match e {
            svg::SvgEvent::Line { x1, x2, y1, y2, stroke, stroke_width } => {
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
            },
            svg::SvgEvent::Text(content) => text::render((width, height), &content, canvas.as_mut_slice()),
            svg::SvgEvent::Circle { fill, cx, cy, r } => {
                let (re_cx, re_cy, re_r);
                let avg;
                {
                    let width = view_box[2] - view_box[0];
                    let height = view_box[3] - view_box[1];
                    re_cx = (cx - view_box[0]) / width;
                    re_cy = (cy - view_box[1]) / height;
                    avg = f64::sqrt(width * height);

                    re_r = r / avg;
                }
                let unit = 1. / f64::sqrt(width as f64 * height as f64);
                for offset in 0 .. width * height {
                    let x = offset % height;
                    let y = offset / height;

                    let re_x = x as f64 / width as f64;
                    let re_y = y as f64 / height as f64;

                    fn square(x: f64) -> f64 {
                        x * x
                    }

                    let sq_distance = square(re_x - re_cx) + square(re_y - re_cy);

                    if sq_distance < square(re_r) {
                        canvas[4 * offset as usize + 0] = fill.red;
                        canvas[4 * offset as usize + 1] = fill.green;
                        canvas[4 * offset as usize + 2] = fill.blue;
                        canvas[4 * offset as usize + 3] = 255;
                        //println!("#{} {}", f64::sqrt(-diff), unit);
                    } else {
                        let factor = (f64::sqrt(sq_distance) - re_r) / unit;
                        if factor < 2. {
                            let gray_scale = if factor >= 1.0 {
                                0.1
                            } else {
                                1. / factor
                            };

                            let r_l = canvas[4 * offset as usize + 0] as f64 / 256.;
                            let g_l = canvas[4 * offset as usize + 1] as f64 / 256.;
                            let b_l = canvas[4 * offset as usize + 2] as f64 / 256.;
                            let a_l = canvas[4 * offset as usize + 3] as f64 / 256.;

                            let r_r = fill.red as f64 / 256.;
                            let g_r = fill.green as f64 / 256.;
                            let b_r = fill.blue as f64 / 256.;

                            let out_a = a_l + gray_scale * (1. - r_l);

                            if out_a > 0. {
                                canvas[4 * offset as usize + 3] = (out_a * 256.) as u8;
                                canvas[4 * offset as usize + 0] = ((r_l * a_l + r_r * gray_scale * (1. - a_l)) / out_a * 256.) as u8;
                                canvas[4 * offset as usize + 1] = ((g_l * a_l + g_r * gray_scale * (1. - a_l)) / out_a * 256.) as u8;
                                canvas[4 * offset as usize + 2] = ((b_l * a_l + b_r * gray_scale * (1. - a_l)) / out_a * 256.) as u8;
                            }


                            //canvas[4 * offset as usize + 3] = 255;
                            //canvas[4 * offset as usize + 0] = 255;
                            //canvas[4 * offset as usize + 1] = 0;
                            //canvas[4 * offset as usize + 2] = 0;
                        }
                    }
                }
            }
        }
    }

    canvas
}

mod distance;
mod text;