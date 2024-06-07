use std::collections::HashSet;

fn swap(x: char) -> char {
    return match x {
        '-' => '+',
        _ => '-'
    }
}

fn transvers (mut v: Vec<Vec<char>>, transversal: Vec<(usize, usize)>) ->  Vec<Vec<char>> {
    let mut rowset: HashSet<usize> = HashSet::from_iter(0..v.len());
    let mut colset: HashSet<usize> = HashSet::from_iter(0..v.len());

    for (i, j) in transversal {
        rowset.remove(&i);
        colset.remove(&j);
        v[i][j] = swap(v[i][j]);
    }

    while rowset.len() > 0 {
        let i = rowset.iter().next().unwrap().clone();
        rowset.remove(&i);
        let j: usize = colset.iter().next().unwrap().clone();
        colset.remove(&j);

        v[i][j] = swap(v[i][j]);
    }

    v
}

fn sq_swap(mut v: Vec<Vec<char>>, t: usize, b: usize, l: usize, r: usize) -> Vec<Vec<char>> {
    v[t][l] = swap(v[t][l]);
    v[t][r] = swap(v[t][r]);
    v[b][l] = swap(v[b][l]);
    v[b][r] = swap(v[b][r]);

    v
}

fn print_matrix(v: &Vec<Vec<char>>) {
    for ln in v {
        println!("{:?}", ln);
    }
}

fn count_pluses(v: &Vec<Vec<char>>) -> u8 {
    let mut pluses_count = 0;
    for line in v {
        for ch in line {
            if *ch == '+' {
                pluses_count += 1;
            }
        }
    }

    pluses_count
}

fn main() {
    let n = 7;
    let mut vec = vec![
        vec!['+', '-', '+', '+', '+', '+', '+'],
        vec!['-', '+', '-', '-', '-', '-', '-'],
        vec!['-', '-', '-', '-', '-', '-', '-'],
        vec!['-', '-', '-', '-', '-', '-', '-'],
        vec!['-', '-', '-', '-', '-', '-', '-'],
        vec!['-', '-', '-', '-', '-', '-', '-'],
        vec!['-', '-', '-', '-', '-', '-', '-'],
    ];

    println!("Input");
    print_matrix(&vec);

    println!("Cornerize");
    for i in 1..n {
        for j in 1..n {
            if vec[i][j] == '+' {
                vec = sq_swap(vec, 0, i, 0, j);
                print_matrix(&vec);
                println!()
            }
        }
    }

    println!("Diag");

    let mut transversal = Vec::from([(0, 0)]);

    for i in 1..n {
        if vec[i][0] == '+' {
            for j in 1..n {
                if vec[0][j] == '+' {
                    transversal.push((i, j));
                    vec = sq_swap(vec, 0, i, 0, j);
                    print_matrix(&vec);
                    println!();
                    break;
                }
            }
        }
    }

    let pluses = count_pluses(&vec);
    println!("{}", pluses);

    if pluses >= n as u8 {
        println!("Transvers:");
        vec = transvers(vec, transversal);

        let pluses = count_pluses(&vec);
        print_matrix(&vec);
        println!("{}", pluses);

        if pluses >= n as u8 {
            let mut pair: Vec<usize> = Vec::new();
            
            println!("Last step:");
            for j in 0..vec.len() {
                if vec[0][j] == '+' {
                    pair.push(j)
                }

                if pair.len() == 2 {
                    for i in 1..vec.len() {
                        if vec[i][j] == '+' {
                            vec = sq_swap(vec, 0, i, pair[0], pair[1]);
                            pair.remove(1);
                            pair.remove(0);

                            print_matrix(&vec);
                            println!();


                            break;
                        }
                    }

                }
            }
        }
    }



    println!("Result");
    print_matrix(&vec)
}
