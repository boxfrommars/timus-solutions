use std::{io, f64::consts::PI};

fn main() {
    let mut input = String::new();
    let mut args = Vec::new();

    io::stdin().read_line(&mut input).unwrap();
    for x in input
        .trim()
        .split(' ')
        .map(|a| a.parse::<f64>().expect("integer")) {
            args.push(x)
        };
    
    let rope_length = args.pop().expect("has rope length argument");
    let garden_length = args.pop().expect("has length argument");

    let half_garden = garden_length / 2.;
    let half_diag = half_garden * 2_f64.sqrt();

    if rope_length >= half_diag {
        // rope is long enough to reach every point of the garden
        return println!("{:.3}", garden_length.powi(2))
    }
    
    if rope_length <= half_garden {
        // rope is too short and it prevents reaching sides of the garden
        return println!("{:.3}", PI * rope_length.powi(2))
    } 
    
    // rope intersects sides of the gerden, area that reachable consists of triangle area, where rope intersects
    // side, and sector area where doesn't
    let half_intersect_angle = (half_garden / rope_length).acos();
    let nonintersect_area = rope_length.powi(2) * (PI - half_intersect_angle * 4.);
    let intersect_area = 4. * half_garden * (rope_length.powi(2) - half_garden.powi(2)).sqrt();

    println!("{:.3}", nonintersect_area + intersect_area);
}
