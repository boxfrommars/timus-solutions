use std::io;

fn main() {
    let mut res = Vec::new();

    let mut num = String::new();
    io::stdin().read_line(&mut num).unwrap();
    let mut num = num.trim().parse::<u32>().unwrap();

    // result must be positive number, so for 0 the smallest number is 10
    if num == 0 {
        return println!("10");
    }

    if num < 10 {
        return println!("{}", num);
    }

    // Try to divide num on all digits starting from the biggest
    let mut max_divider = 9;

    while max_divider > 1 {
        if num % max_divider == 0 {
            num /= max_divider;
            res.push(max_divider);
        } else {
            max_divider -= 1;
        }
    }

    // if number can be represented as a product of digits then after dividing on all digits there must be 1
    if num == 1 {
        // Print dividers in the ascending order
        for x in res.iter().rev() {
            print!("{}", x)
        }
        println!()
    } else {
        // return -1 if number cannot be represented as a product of digits
        println!("-1")
    }
}
