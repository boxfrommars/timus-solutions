use std::io;
use std::cmp;
use std::convert::TryFrom;

fn main() {
    let mut stones_string = String::new();

    io::stdin().read_line(&mut String::new()).unwrap(); // Skip first line
    io::stdin().read_line(&mut stones_string).unwrap();  // Weights of the stones, with space as delimeter

    let mut stones: Vec<u32> = Vec::new();

    // Parse weights tring to u32 vec
    for s in stones_string
        .trim()
        .split(' ')
        .map(|a| a.parse::<u32>().expect("integer")) {
            stones.push(s)
        };
    
    // Totaal weight of the stones
    let total: u32 = stones.iter().sum();
    let knapsack_size = total / 2;

    // println!("{}", knapsack_size);
    let mut prev_col = vec![0; usize::try_from(knapsack_size + 1).unwrap()];
    for stone in stones.iter() {
        let mut col: Vec<u32> = Vec::new();
        for (i, prev_col_optimal) in prev_col.iter().enumerate() {
            let current_size = i as u32;
            let without_stone = prev_col_optimal.clone();
            if &current_size >= stone {
                let with_stone = prev_col[usize::try_from(current_size - stone).unwrap()] + stone;
                col.push(cmp::max(with_stone, without_stone));
            } else {
                col.push(without_stone);
            }
        }
        // println!("{:?}", col);
        prev_col = col;
    }

    match prev_col.last() {
         Some(i) => println!("{}", total - 2 * i),
         None => println!("{}", stones.first().expect("has at least one stone"))
    };
}
