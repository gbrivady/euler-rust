fn len_cycle(n: u32) -> u32 {
    // let mut quotients: Vec<u32> = Vec::new();
    let mut reminders: Vec<u32> = vec![0; n as usize];
    let mut r = 1;
    let mut i = 1;
    reminders[1] = 1;

    loop {
        i += 1;
        r *= 10;
        r %= n;
        if r == 0 {
            return 0;
        }
        if reminders[r as usize] != 0 {
            return i - reminders[r as usize];
        }
        reminders[r as usize] = i;
    }
}

fn main() {
    let mut id_c: u32 = 0;
    let mut max_c: u32 = 0;
    for i in 3..1000 {
        let c: u32 = len_cycle(i);
        if i % 2 == 0 || i % 5 == 0 {
            continue;
        }
        if c > max_c {
            id_c = i;
            max_c = c;
        }
    }
    println!("{id_c} {max_c}");
}
