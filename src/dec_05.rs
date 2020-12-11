use std::fs;

pub fn run(){
    let data = fs::read_to_string("input_05.txt").expect("Unable to read file");
    let mut answer = 0;
    for line in data.lines(){
        let (row, col) = line.split_at(7);
        let row: Vec<char> = row.chars().collect();
        let col: Vec<char> = col.chars().collect();
        let mut left = 0;
        let mut right = 127;
        println!("---------------------------");
        let mut seat_row = 0;
        for c in row {
            seat_row = left + (right - left) / 2;
            match c {
                'B' => {
                    left = seat_row + 1;
                    println!("B seat_row = {} left = {} right = {}", seat_row, left, right);
                },
                'F' => {
                    right = seat_row - 1;
                    println!("F seat_row = {} left = {} right = {}", seat_row, left, right);
                },
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
                    println!("R seat_col = {} left = {} right = {}", seat_col, left, right);
                },
                'L' => {
                    right = seat_col - 1;
                    println!("L seat_col = {} left = {} right = {}", seat_col, left, right);
                },
                _ => println!("Input error R/L")
            }
        }
        if left == right {
            seat_col = right;
        }

        let seat = seat_row * 8 + seat_col;
        println!("Seat = {} Row = {} Col = {}", seat, seat_row, seat_col);
        if answer < seat {
            answer = seat_row * 8 + seat_col
        }
    }
    println!("Seat = {}", answer);
}