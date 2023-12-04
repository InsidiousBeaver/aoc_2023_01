use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let input_path = env::var("aoc_2023_01_path").unwrap() + "/input.txt";
    let mut sum = 0u64;
    let numbers_dict = [
        (1u64, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
    ];
    let input_file = File::open(input_path).unwrap();
    let reader = BufReader::new(input_file);
    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut first: Option<u64> = Option::None;
        let mut last: Option<u64> = Option::None;
        let mut word = String::with_capacity(line.len());
        println!("{}", line);

        for c in line.chars() {
            if c.is_ascii_digit() {
                if first.is_none() {
                    first = Some(u64::from_str_radix(&c.to_string(), 10).unwrap());
                    last = Some(u64::from_str_radix(&c.to_string(), 10).unwrap());
                } else {
                    last = Some(u64::from_str_radix(&c.to_string(), 10).unwrap());
                }
            } else {
                word.push(c);
                for (n, str_val) in numbers_dict {
                    if word.contains(str_val) {
                        let last_char = str_val.chars().last().unwrap();
                        word = word.replace(str_val, &last_char.to_string());
                        if first.is_none() {
                            first = Some(n);
                            last = Some(n);
                        } else {
                            last = Some(n);
                        }
                    }
                }
            }
        }
        if first.is_some() && last.is_some() {
            let result = first.unwrap().to_string() + &last.unwrap().to_string();
            sum = sum + u64::from_str_radix(&result, 10).unwrap();
        }
    }
    println!("{}", sum);
}
