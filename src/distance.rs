
fn between(delta: f64, x: f64, (x1, x2): (f64, f64)) -> bool {
    (x < x1 + delta && x > x2 - delta) || (x > x1 - delta && x < x2 + delta)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert!(super::between(0.01, 1.0, (0.9, 1.1)));
        assert!(super::hit(0.01, (0.5, 0.0, 0.5, 0.5), 0.5, 0.5));
        assert!(!super::hit(0.01, (0.5, 0.0, 0.5, 0.5), 0.0, 0.0));
        assert!(!super::hit(0.01, (0.5, 0.0, 0.5, 0.5), 0.5, 1.0));
    }
    #[test]
    fn test1() {
        assert!(super::hit(0.01, (0.5, 0.0, 0.5, 1.0), 0.5, 0.5));
    }
    #[test]
    fn test2() {
        assert!(super::between(0.01, 0.5, (0.5, 0.5)));
        assert!(super::between(0.01, 0.5, (0.0, 1.0)));
    }
}

pub fn distance(delta: f64, (x1, y1, x2, y2): (f64, f64, f64, f64), x0: f64, y0: f64) -> f64 {
    fn square(x: f64) -> f64 { x * x }

    if between(delta, x0, (x1, x2)) && between(delta, y0, (y1, y2)) {
        if f64::abs(x1 - x2) < delta {
            f64::abs(x0 - (x1 + x2) / 2.0)
        } else {
            f64::abs((y2 - y1) * x0 - (x2 - x1) * y0 + x2 * y1 - y2 * x1) / f64::sqrt(square(y2 - y1) + square(x2 - x1))
        }
    } else {
        ::std::f64::INFINITY
    }            
}
pub fn hit(delta: f64, line: (f64, f64, f64, f64), x0: f64, y0: f64) -> bool {
    let ret = distance(delta, line, x0, y0) < delta;
    //println!("{:?} {} {} {}", line, x0, y0, ret);
    ret
}
