extern crate advtools;
use advtools::prelude::*;

type Values = (i32, i32, i32, i32, i32);

fn add_up<F>(amounts: &[i32], v: &[Values], select: F) -> i32 where F: Fn(&Values) -> i32 {
    amounts.iter().enumerate().map(|(i, a)| a*select(&v[i])).sum()
}

fn fom(amounts: &[i32], v: &[Values]) -> i32 {
    add_up(amounts, v, |vi| vi.0).max(0) * add_up(amounts, v, |vi| vi.1).max(0) *
        add_up(amounts, v, |vi| vi.2).max(0) * add_up(amounts, v, |vi| vi.3).max(0)
}

fn gen_amounts(sum: usize, n: usize) -> Vec<Vec<i32>> {
    if n == 1 {
        vec![vec![sum as i32]]
    } else {
        let mut res = Vec::new();
        for a in 1..sum-n {
            for mut amnts in gen_amounts(sum - a, n - 1) {
                amnts.push(a as i32);
                res.push(amnts);
            }
        }
        res
    }
}

fn find_best(goalcal: Option<i32>, v: &[Values]) -> Vec<i32> {
    gen_amounts(100, v.len())
        .into_iter()
        .filter(|amounts| if let Some(goal) = goalcal {
            add_up(amounts, v, |vi| vi.4) == goal
        } else { true })
        .max_by_key(|amounts| fom(amounts, v)).unwrap()
}

fn main() {
    let mut v = Vec::new();
    for tok in iter_input::<Vec<String>>() {
        v.push((to_i32(tok[2].trim_matches(',')),
                to_i32(tok[4].trim_matches(',')),
                to_i32(tok[6].trim_matches(',')),
                to_i32(tok[8].trim_matches(',')),
                to_i32(&tok[10])));
    }
    let best = find_best(None, &v);
    println!("Best: {:?} -> {}", best, fom(&best, &v));
    let new_best = find_best(Some(500), &v);
    println!("Best with 500 cal: {:?} -> {}", new_best, fom(&new_best, &v));
}
