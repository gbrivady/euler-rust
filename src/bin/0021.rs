fn main() {
    let mut sum_divisors: [u32; 10000] = [0; 10000];
    for i in 1..10000usize {
        for k in (2..100u32).take_while(|n| (*n).pow(2) < i as u32) {
            if i as u32 % k == 0 {
                sum_divisors[i] += (i as u32 / k) + i as u32 / (i as u32 / k);
            }
        }
        sum_divisors[i] += 1;
    }

    let mut sum: u32 = 0;
    for i in 1..10000 {
        let d = sum_divisors[i];
        if d < 10000 && d != i as u32 && sum_divisors[d as usize] == i as u32 {
            sum += i as u32;
        }
    }
    println!("{sum}");
}
