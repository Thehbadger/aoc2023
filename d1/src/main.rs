use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

fn main() {
    let data_path = Path::new("data.txt");
    let fd = File::open(data_path).unwrap();
    let buff_reader = BufReader::new(fd);
    let vals: Vec<String> = buff_reader.lines().map(|x| x.unwrap()).collect();
    let result: u32 = vals.iter().map(|x| process_val(&x)).sum();
    println!("Total: {}", result);
}

fn process_val(input: &str) -> u32 {
    let chars: Vec<char> = input.chars().filter(|x| x.is_ascii_digit()).collect();
    if chars.len() == 0 {
        return 0;
    } else {
        let first = chars.first().unwrap();
        let last = chars.last().unwrap();
        let mut combined: String = String::new();
        combined.push(*first);
        combined.push(*last);
        combined.parse().unwrap()
    }
}
