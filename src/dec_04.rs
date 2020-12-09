use std::fs;

use hex::*;

pub fn run() {
    let data = fs::read_to_string("input_04.txt").expect("Unable to read file");
    let mut field_count = 0;
    let mut cid_count = 0;
    let mut valid_count = 0;

    for line in data.lines() {
        for word in line.split_whitespace() {
            let (field, mut data) = word.split_at(3);
            data = data.split_at(1).1;
            //println!("Field {}  Data {}", field, data);
            match field {
                "byr" => match data.parse::<u16>() {
                    Ok(number) => {
                        if number <= 2002 && 1920 <= number {
                            field_count += 1;
                        } else {
                            println!("byr out of scope {}", number);
                        }
                    }
                    Err(e) => println!("byr error {}", e),
                },
                "iyr" => match data.parse::<u16>() {
                    Ok(number) => {
                        if number <= 2020 && 2010 <= number {
                            field_count += 1;
                        } else {
                            println!("iyr out of scope {}", number);
                        }
                    }
                    Err(e) => println!("iyr error {}", e),
                },
                "eyr" => match data.parse::<u16>() {
                    Ok(number) => {
                        if number <= 2030 && 2020 <= number {
                            field_count += 1;
                        } else {
                            println!("eyr out of scope {}", number);
                        }
                    }
                    Err(e) => println!("byr error {}", e),
                },
                "hgt" => {
                    let (number, metric) = data.split_at(data.len() - 2);
                    match number.parse::<u16>() {
                        Ok(number) => match metric {
                            "cm" => {
                                if number <= 193 && 150 <= number {
                                    field_count += 1
                                }
                            }
                            "in" => {
                                if number <= 76 && 59 <= number {
                                    field_count += 1
                                }
                            }
                            _ => println!("hgt error {}", data),
                        },
                        Err(e) => println!("hgt numeric error {}", e),
                    }
                }
                "hcl" => {
                    let (hashtag, hex) = data.split_at(1);
                    if hashtag == "#" {
                        match Vec::from_hex(hex) {
                            Ok(_) => {
                                field_count += 1;
                            }
                            Err(e) => println!("Hex error {} {}", e, data),
                        }
                    } else{
                        println!("Hashtag error {}", word);
                    }
                }
                "ecl" => match data {
                    "amb" => field_count += 1,
                    "blu" => field_count += 1,
                    "brn" => field_count += 1,
                    "gry" => field_count += 1,
                    "grn" => field_count += 1,
                    "hzl" => field_count += 1,
                    "oth" => field_count += 1,
                    _ => println!("ecl error {} ", word),
                },
                "pid" => {
                    if data.chars().count() == 9 {
                        match data.parse::<u32>() {
                            Ok(_) => {
                                field_count += 1;
                            }
                            Err(e) => println!("Hexadecimal error {}", e),
                        }
                    }
                }
                _ => cid_count += 1,
            }
        }
        if line.is_empty() {
            if field_count == 7 {
                valid_count += 1;
            }
            field_count = 0;
        }
    }
    println!("valid_count {}", valid_count);
}
