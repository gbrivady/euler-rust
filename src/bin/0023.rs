fn is_abundant(n: u32) -> bool {
    let mut sum_dividers = 1;
    for i in (2..168u32).take_while(|x| (*x).pow(2) <= n) {
        if n % i == 0 {
            let a = n / i;
            let b = n / a;
            if a == b {
                sum_dividers += a;
            } else {
                sum_dividers += (n / i) + n / (n / i);
            }
        }
    }
    return sum_dividers > n;
}

fn main() {
    let mut is_sum_of_abundants = [false; 28123];

    let mut abundants: Vec<u32> = Vec::new();

    for i in 1..28123 {
        if is_abundant(i) {
            abundants.push(i);
            for n in abundants.iter() {
                let m = n + i;
                if m < 28123 {
                    is_sum_of_abundants[m as usize] = true;
                }
            }
        }
    }
    println!(
        "{}",
        is_sum_of_abundants
            .iter()
            .enumerate()
            .fold(0u32, |acc, (i, is_sum)| acc
                + (i as u32) * (1 - *is_sum as u32))
    )
}
