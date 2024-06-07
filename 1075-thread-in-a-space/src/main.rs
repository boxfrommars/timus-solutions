use std::{io::{self, BufRead}, f64::consts::PI};

fn dist(p1: &[i32], p2: &[i32]) -> f64 {
    let mut res: f64 = 0.;

    for (i, _v) in p1.iter().enumerate() {
        res += (p1[i] - p2[i]).pow(2) as f64;
    }

    res = res.sqrt();

    res
}

fn parse_vec(s: String) -> Vec<i32> {
    s.trim()
        .split(' ')
        .map(|a| a.parse().expect("integer"))
        .collect()
}

fn parse_num(s: String) -> f64 {
    s.trim().parse().expect("numeric")
}

fn main() {
    let mut input = Vec::new();
    for s in io::stdin().lock().lines() {
        input.push(s.expect("Has line"));
    }

    let r = parse_num(input.pop().unwrap());
    let c_point = parse_vec(input.pop().unwrap());
    let b_point = parse_vec(input.pop().unwrap());
    let a_point = parse_vec(input.pop().unwrap());

    let a = dist(&b_point, &c_point);
    let b = dist(&a_point, &c_point);
    let c = dist(&a_point, &b_point);

    if c == 0. {
        return println!("0");
    }

    let alpha1 = (r / a).acos();
    let alpha2 = (r / b).acos();

    let mut c_angle = PI;

    if (a + b - c).abs() > 1e-5 {
        c_angle = ((a.powi(2) + b.powi(2) - c.powi(2)) / (2. * a * b)).acos();
    }

    if c_angle < (alpha1 + alpha2) {
        return println!("{:.2}", c);
    }

    let res = (a.powi(2) - r.powi(2)).sqrt()
        + (b.powi(2) - r.powi(2)).sqrt()
        + r * (c_angle - alpha1 - alpha2);

    println!("{:.2}", res);
}
