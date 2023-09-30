use std::vec;

fn padic_valuation(n: &u64, p: &u64) -> u64 {
    let mut vp: u64 = 0;
    let mut p_pow = 1;
    loop {
        p_pow *= p;
        let r = n % p_pow;
        if r != 0 {
            break vp;
        }
        vp += 1
    }
}

fn get_p_values(n: u64, primes: &Vec<u64>) -> Vec<u64> {
    let mut p_values: Vec<u64> = Vec::new();
    for p in primes.iter() {
        p_values.push(padic_valuation(&n, p));
    }
    return p_values;
}

fn main() {
    let mut primes: Vec<u64> = vec![2];

    let mut last_pvals: Vec<u64> = vec![0];
    for n in 3..13000 {
        let pvals: Vec<u64> = get_p_values(n, &primes);
        if pvals.iter().sum::<u64>() == 0 {
            primes.push(n);
            last_pvals.push(0);
        }
        let nb_div_tn = pvals
            .iter()
            .zip(last_pvals.iter())
            .fold(1u64, |acc, (vp1, vp2)| acc * (vp1 + vp2 + 1));
        if nb_div_tn >= 500 {
            let tn = ((n - 1) * n) / 2;
            println!(
                "{}, T_{} has {} dividers. {}",
                tn,
                n - 1,
                get_p_values(tn, &primes)
                    .iter()
                    .fold(1u64, |acc, vp| acc * (vp + 1)),
                nb_div_tn
            );
            // break;
        }
        last_pvals = pvals;
    }
}
