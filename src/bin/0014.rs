fn main() {
    let mut cycle_len: Vec<u64> = vec![1];

    for i in 2..1000000 {
        let mut u: u64 = i;
        let mut n: u64 = 1;
        cycle_len.push(loop {
            if u < i {
                break n + cycle_len[(u - 1) as usize] - 1;
            }
            if u == 1 {
                break n;
            }
            if u % 2 == 0 {
                u /= 2;
            } else {
                u = 3 * u + 1;
            }
            n += 1;
        });
    }
    let mut n_max: u64 = 0;
    let mut max_cycle: u64 = 0;

    for (c, i) in cycle_len.iter().zip(0..cycle_len.len()) {
        if *c > max_cycle {
            max_cycle = *c;
            n_max = (i + 1) as u64;
        }
    }
    println!("{} {}", n_max, max_cycle);
}
