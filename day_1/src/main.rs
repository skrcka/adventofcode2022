use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    const FILEPATH: &str = "./input.txt";
    let file = File::open(FILEPATH).unwrap();
    let reader = BufReader::new(file);

    let mut data = Vec::new();
    data.push(0);
    let mut i = 0;

    for line in reader.lines().map(|l| l.unwrap()) {
        if line.len() == 0 {
            i+=1;
            data.push(0);
            continue;
        }
        data[i] += line.parse::<i32>().unwrap();
    }
    data.sort();
    println!("{}", data.iter().rev().take(3).sum::<i32>());
}
