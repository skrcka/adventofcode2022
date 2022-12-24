use std::fs::File;
use std::collections::HashSet;
use std::io::{BufRead, BufReader};

fn main() {
    const FILEPATH: &str = "./input.txt";
    let file = File::open(FILEPATH).unwrap();
    let reader = BufReader::new(file);
    let mut priorities: u32 = 0;

    /* part 1
    for line in reader.lines().map(|l| l.unwrap()) {
        let (first, second) = line.split_at(line.len()/2);
        
        let set: HashSet<char> = first.chars().collect();
        for ch in set {
            if second.chars().any(|c| c == ch) {
                priorities += if ch.is_uppercase() {ch as u32 - 'A' as u32 + 27} else {ch as u32 - 'a' as u32 + 1};
            }
        }
    }
    */
    let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    for i in (0..lines.len()).step_by(3) {
        let first = &lines[i];
        let second = &lines[i+1];
        let third = &lines[i+2];
        
        for ch in first.chars() {
            if second.chars().any(|c| c == ch) {
                if third.chars().any(|c| c == ch) {
                    priorities += if ch.is_uppercase() {ch as u32 - 'A' as u32 + 27} else {ch as u32 - 'a' as u32 + 1};
                    break;
                }
            }
        }
    }
    println!("{}", priorities);
}
