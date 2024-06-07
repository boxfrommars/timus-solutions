use std::io::{self, BufReader};
use std::io::prelude::*;

fn main() {
    let reader = BufReader::new(io::stdin());
    let mut out = io::stdout();
    let mut v: Vec<i64> = Vec::new();
    for t in reader.lines().map(|a| a.expect("correct input")) {
        for u in t.split(' ')
            .map(|s| s.trim())
            .filter(|a| a.len() > 0)
            .map(|a| a.parse::<i64>().expect("integer")) {
                v.push(u)
            }
    }

    for u in v.into_iter().rev().map(|a| a as f64) {
        writeln!(out, "{:.*}", 4, u.sqrt()).expect("valid output");
    }
}
