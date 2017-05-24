
fn between(x: f64, (x1, x2): (f64, f64)) -> bool {
    (x < x1 + 0.01 && x > x2 - 0.01) || (x > x1 - 0.01 && x < x2 + 0.01)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert!(super::between(1.0, (0.9, 1.1)));
        assert!(super::hit((0.5, 0.0, 0.5, 0.5), 0.5, 0.5));
        assert!(!super::hit((0.5, 0.0, 0.5, 0.5), 0.0, 0.0));
        assert!(!super::hit((0.5, 0.0, 0.5, 0.5), 0.5, 1.0));
    }
    #[test]
    fn test1() {
        assert!(super::hit((0.5, 0.0, 0.5, 1.0), 0.5, 0.5));
    }
    #[test]
    fn test2() {
        assert!(super::between(0.5, (0.5, 0.5)));
        assert!(super::between(0.5, (0.0, 1.0)));
    }
}

pub fn distance((x1, y1, x2, y2): (f64, f64, f64, f64), x0: f64, y0: f64) -> f64 {
    fn square(x: f64) -> f64 { x * x }

    if between(x0, (x1, x2)) && between(y0, (y1, y2)) {
        if f64::abs(x1 - x2) < 0.02 {
            f64::abs(x0 - (x1 + x2) / 2.0)
        } else {
            f64::abs((y2 - y1) * x0 - (x2 - x1) * y0 + x2 * y1 - y2 * x1) / f64::sqrt(square(y2 - y1) + square(x2 - x1))
        }
    } else {
        ::std::f64::INFINITY
    }            
}
pub fn hit(line: (f64, f64, f64, f64), x0: f64, y0: f64) -> bool {
    let ret = distance(line, x0, y0) < 0.01;
    //println!("{:?} {} {} {}", line, x0, y0, ret);
    ret
}