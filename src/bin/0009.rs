fn main() {
    for i in 1..200u64 {
        for j in 1u64..i {
            if i % j != 0 {
                continue;
            }
            let (a, b, c): (u64, u64, u64) = (i.pow(2) - j.pow(2), 2 * i * j, i.pow(2) + j.pow(2));
            if a + b + c == 1000 {
                println!("{}, {}, {}, {}", a, b, c, a * b * c);
            }
        }
    }
}
