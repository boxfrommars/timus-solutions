use std::io;

fn binomial(mut n: u32, k: u32) -> u32 {
    // See http://blog.plover.com/math/choose.html for the idea.
    if k > n {
        return 0;
    }
    if k > n - k {
        return binomial(n, n - k);
    }
    let mut r = 1;
    let mut d = 1;
    loop {
        if d > k {
            break;
        }
        r *= n;
        r /= d;
        n -= 1;
        d += 1;
    }
    r
}

fn main() {
    let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23];
    let primes_mult = [6, 10, 14, 15, 21, 22];

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

    let max_int = args.pop().expect("has 2nd argument");
    let size = args.pop().expect("has 1st argument");

    let max_div = max_int / size;

    let mut res = 0;

    for p in primes {
        if p <= max_div {
            let numbers_with_p_divider = max_int / p;
            let sets_count = binomial(numbers_with_p_divider, size);
            res += sets_count;
        }
    }

    for pm in primes_mult {
        if pm <= max_div {
            let numbers_with_pm_divider = max_int / pm;
            let sets_count = binomial(numbers_with_pm_divider, size);
            res -= sets_count;
        }
    }

    if res > 10000 {
        res = 10000;
    }

    println!("{}", res);
}
