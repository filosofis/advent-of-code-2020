use std::fs;

pub fn run() {
    let data = fs::read_to_string("input_05.txt").expect("Unable to read file");
    let mut answer = 0;
    let mut seat_vec: Vec<i32> = Vec::with_capacity(128 * 8);

    for line in data.lines() {
        let (row, col) = line.split_at(7);
        let row: Vec<char> = row.chars().collect();
        let col: Vec<char> = col.chars().collect();
        let mut left = 0;
        let mut right = 127;
        let mut seat_row = 0;

        for c in row {
            seat_row = left + (right - left) / 2;
            match c {
                'B' => {
                    left = seat_row + 1;
                }
                'F' => {
                    right = seat_row;
                }
                _ => println!("Input error B/F"),
            }
        }
        if left == right {
            seat_row = right;
        }

        right = 7;
        left = 0;
        let mut seat_col = 0;

        for c in col {
            seat_col = left + (right - left) / 2;
            match c {
                'R' => {
                    left = seat_col + 1;
                }
                'L' => {
                    right = seat_col;
                }
                _ => println!("Input error R/L"),
            }
        }
        if left == right {
            seat_col = right;
        }

        let seat = seat_row * 8 + seat_col;
        if answer < seat {
            answer = seat_row * 8 + seat_col
        }
        seat_vec.push(seat);
    }

    println!("Answer 1) {}", answer);

    seat_vec.sort();
    let mut last = seat_vec[0];
    for seat_id in seat_vec {
        if seat_id - last > 1 {
            println!("Answer 2) {}", last + 1);
            break;
        }
        last = seat_id;
    }
}
