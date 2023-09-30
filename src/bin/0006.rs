fn main() {
    let a: u64 = ((50u64) * (101u64)).pow(2);
    let b: u64 = (100 * 101 * 201) / 6;
    println!("{} {} {}", a, b, a - b);
}
