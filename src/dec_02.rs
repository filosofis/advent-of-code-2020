use std::fs;

pub fn run() {
    let data = fs::read_to_string("input_02.txt").expect("Unable to read file");
    let mut valid_passwords = 0;
    let mut valid_passwords_2 = 0;

    for line in data.lines() {
        let mut words = line.split_whitespace();
        let mut range = words.next().unwrap().split("-");
        let range_min: usize = range.next().unwrap().parse().unwrap();
        let range_max: usize = range.next().unwrap().parse().unwrap();
        let letter = words.next().unwrap().chars().next().unwrap();
        let password = words.next().unwrap();
        let mut sum = 0;

        for character in password.chars() {
            if character == letter {
                sum += 1;
            }
        }

        if range_min <= sum && range_max >= sum {
            valid_passwords += 1;

            // println!(
            //     "range_min {}, range_max {}, letter {}, password {}, sum {}, valid {}",
            //     range_min, range_max, letter, password, sum, valid_passwords
            // );
        }

        let char_vec: Vec<char> = password.chars().collect();
        if (char_vec[range_min-1] == letter) ^ (char_vec[range_max-1] == letter){
                valid_passwords_2 += 1;
            }
    }
    println!("Rule 1 valid passwords: {}", valid_passwords);
    println!("Rule 2 valid passwords: {}", valid_passwords_2);
}
