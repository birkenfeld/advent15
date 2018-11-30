extern crate advtools;
use advtools::prelude::*;

const NEEDLE: &str = "\
children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1
";

fn main() {
    let mut needle = HashSet::default();
    let mut needle_map = HashMap::default();
    for line in NEEDLE.lines() {
        let (name, count): (String, i32) = parse_parts_trim(&line, (0, 1), ":");
        needle.insert((name.clone(), count));
        needle_map.insert(name, count);
    }
    let mut haystack = Vec::new();
    for tok in iter_input_trim::<Vec<(String, i32)>>(":,") {
        haystack.push(HashSet::from_iter(tok.into_iter().skip(1)));
    }
    for (i, hay) in haystack.into_iter().enumerate() {
        if hay.is_subset(&needle) {
            println!("Preliminary aunt: {}", i+1);
        }
        let all_ok = hay.iter().all(|&(ref name, count)| {
            match &**name {
                "cats" | "trees" => count > needle_map[name],
                "pomeranians" | "goldfish" => count < needle_map[name],
                _ => count == needle_map[name]
            }
        });
        if all_ok {
            println!("Real aunt: {}", i+1);
        }
    }
}
