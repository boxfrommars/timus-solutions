use std::io;

fn main() {
    let mut input = String::new();
    let mut args = Vec::new();

    io::stdin().read_line(&mut input).unwrap();
    for x in input
        .trim()
        .split(' ')
        .map(|a| a.parse::<u32>().expect("integer"))
    {
        args.push(x)
    }

    let horse_lifetime = args.pop().unwrap();
    let ilya_nearest = args.pop().unwrap() - 1;
    let alyosha_nearest = args.pop().unwrap() - 1;
    let _total_dist = args.pop().unwrap();
    let ilya_dist_to_sea = args.pop().unwrap();
    let alyosha_dist_to_sea = args.pop().unwrap();

    // Every port can be imagined as based 2 number, where 1 on the n-th position means turn to the left (0 - to the left).
    // Find turn differences (bitwise xor). 1 on the n-th position means that on this turn there was a difference
    let mut turn_differences = ilya_nearest ^ alyosha_nearest;

    // Find the first turn difference
    let mut msb = 0;
    while turn_differences > 0 {
        turn_differences >>= 1;
        msb += 1;
    }

    msb = msb.max(ilya_dist_to_sea).max(alyosha_dist_to_sea);

    let distance = 2 * msb - ilya_dist_to_sea - alyosha_dist_to_sea;

    if distance <= horse_lifetime {
        println!("YES");
    } else {
        println!("NO");
    }
}
