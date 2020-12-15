use std::fs;

pub fn run() {
    let data = fs::read_to_string("input_06.txt").expect("Unable to read file");
    let mut unique_answers: Vec<char> = Vec::with_capacity(128);
    let mut count = 0;

    for line in data.lines() {
        let chars: Vec<char> = line.chars().collect();
        for c in chars {
            if unique_answers.contains(&c) {
            } else {
                unique_answers.push(c);
            }
        }
        if line.is_empty() {
            count += unique_answers.len();
            unique_answers.clear();
        }
    }
    count += unique_answers.len();
    println!("count {}", count);
}
