fn main() {
    let mut year: u32 = 1901;

    let mut day: u32 = 1;

    let mut nb_sunday_first: u32 = 0;

    let mut nb_days: Vec<u32> = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    while year < 2001 {
        nb_days[1] = 28 + ((year % 4) == 0) as u32;
        // January
        for m in nb_days.iter() {
            if day == 6 {
                nb_sunday_first += 1;
            }
            day = (day + m) % 7;
        }

        year += 1;
    }
    println!("{}", nb_sunday_first);
}
