use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let res = input
        .trim()
        .split(' ')
        .map(|a| a.parse::<i32>())
        .map(|a| a.unwrap())
        .fold(0i32, |sum, a| sum + a);

    println!("{}", res);
}
