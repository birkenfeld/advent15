extern crate advtools;

use std::cmp::min;

const INPUT: u32 = 2503;

fn main() {
    let mut deer = Vec::new();
    for tok in advtools::iter_input::<Vec<String>>() {
        let fly_time = tok[6].parse::<u32>().unwrap();
        deer.push((tok[0].to_owned(), tok[3].parse::<u32>().unwrap(),
                   fly_time, fly_time + tok[13].parse::<u32>().unwrap(),
                   0, 0));
    }

    let winner = deer.iter().map(|&(ref name, speed, fly_time, cycle_time, _, _)| {
        let cycles = INPUT / cycle_time;
        let rest_time = INPUT % cycle_time;
        (name.clone(), speed * (cycles * fly_time + min(rest_time, fly_time)))
    }).max_by_key(|v| v.1).unwrap();
    println!("Traditional: {} ({} km)", winner.0, winner.1);

    for time in 0..INPUT {
        let best = deer.iter_mut().map(|&mut (_, speed, fly_time, cycle_time, ref mut dist, _)| {
            if time % cycle_time < fly_time {
                *dist += speed;
            }
            *dist
        }).max().unwrap();
        for &mut (_, _, _, _, dist, ref mut points) in &mut deer {
            if dist == best {
                *points += 1;
            }
        }
    }
    let winner = deer.iter().max_by_key(|v| v.5).unwrap();
    println!("New-style: {} ({} points)", winner.0, winner.5);
}