use std::fs;

pub fn run() {
    let data = fs::read_to_string("input_03.txt").expect("Unable to read file");

    let answer: u32 = slope(1, 1, &data)
        * slope(3, 1, &data)
        * slope(5, 1, &data)
        * slope(7, 1, &data)
        * slope(1, 2, &data);

    println!("answer {}", answer);
}

pub fn slope(x: usize, y: usize, data: &String) -> u32 {
    let mut tree_count = 0;
    let mut vec = [0, 0];
    let mut lines = data.lines().step_by(y);

    loop {
        match lines.next() {
            Some(line) => {
                let chars: Vec<char> = line.chars().collect();

                if chars[vec[0]] == '#' {
                    tree_count += 1;
                }
                vec[0] += x;
                if vec[0] > 30 {
                    vec[0] -= 31;
                }

                vec[1] += 1;
            }
            None => break,
        }
    }
    tree_count
}
