extern crate advtools;
use advtools::prelude::*;

fn main() {
    let (total_paper, total_ribbon) = iter_input::<String>().fold(
        (0, 0), |(paper, ribbon), line| {
            let dims = sorted(line.split('x').map(to_u32));
            let (l, w, h) = (dims[0], dims[1], dims[2]);
            (paper + 2 * (l*w + w*h + h*l) + l*w, ribbon + l*w*h + 2 * (l + w))
        });
    println!("Paper: {}", total_paper);
    println!("Ribbon: {}", total_ribbon);
}
