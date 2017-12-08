const INPUT: &[u8] = b"1321131112";

fn push_pair(v: &mut Vec<u8>, n: u8, d: u8) {
    if n >= 100 {
        v.push(n / 100);
        v.push((n % 100) / 10);
        v.push(n % 10);
    } else if n >= 10 {
        v.push(n / 10);
        v.push(n % 10);
    } else {
        v.push(n);
    }
    v.push(d);
}

fn main() {
    let mut seq: Vec<u8> = INPUT.iter().map(|&ch| ch - b'0').collect();
    for i in 1..51 {
        let mut new_seq = Vec::with_capacity(2 * seq.len());
        let mut dp = seq[0];
        let mut n = 0;
        for d in seq {
            if d != dp {
                push_pair(&mut new_seq, n, dp);
                n = 0;
            }
            n += 1;
            dp = d;
        }
        push_pair(&mut new_seq, n, dp);
        seq = new_seq;
        if i > 35 && i % 10 == 0 {
            println!("Resulting length after {} iterations: {}", i, seq.len());
        }
    }
}
