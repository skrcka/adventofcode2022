use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    const FILEPATH: &str = "./input.txt";
    let file = File::open(FILEPATH).unwrap();
    let reader = BufReader::new(file);

    let mut score: u32 = 0;

    for line in reader.lines().map(|l| l.unwrap()) {
        let mut parts = line.split_whitespace();
        let first = parts.next().unwrap().parse::<char>().unwrap() as u32 - 'A' as u32 + 1;
        let second = parts.next().unwrap().parse::<char>().unwrap() as u32 - 'X' as u32 + 1;
        
        /* part 1
        score += second;
        match second as i32 - first as i32 {
            1=> score += 6,
            0=> score += 3,
            -2=> score += 6,
            _=> score += 0,
        }
        */

        if second == 1 {
            score += if first != 1 {first - 1} else {3};
        }
        else if second == 2 {
            score += 3;
            score += first;
        }
        else {
            score += 6;
            score += if first != 3 {first + 1} else {1};
        }
    }
    println!("{}", score);
}
