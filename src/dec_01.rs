use std::fs;

pub fn run() {
    let data = fs::read_to_string("input_01.txt").expect("Unable to read file");

    for num_a in data.lines() {
        for num_b in data.lines() {
            for num_c in data.lines() {
                let a: u32 = num_a.parse().unwrap();
                let b: u32 = num_b.parse().unwrap();
                let c: u32 = num_c.parse().unwrap();
                if a + b + c == 2020 {
                    println!("{} + {} + {} = 2020, sum = {}", 
                    num_a, num_b, num_c, a * b * c);
                    return;
                }
            }
        }
    }
}
