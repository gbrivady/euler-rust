use std::env::args;

fn main() {
    let mut primes: Vec<u64> = Vec::<u64>::new();
    let mut i = 2;

    while primes.len() < args().collect::<Vec<String>>()[1].parse().unwrap() {
        let mut v = primes.iter();
        if loop {
            match v.next() {
                None => break true,
                Some(n) => {
                    if i % n == 0 {
                        break false;
                    }
                }
            }
        } {
            primes.push(i);
        }
        i += 1;
    }
    print!("{}", primes.last().unwrap());
}
