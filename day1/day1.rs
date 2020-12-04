use std::fs;
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
    let numbers_str  = contents.lines();
    let numbers: HashSet<i32> = numbers_str.map(|s| s.parse().unwrap()).collect();
    let target: i32 = 2020;

    for number in &numbers {
        let needed: i32 = target - number;
        if numbers.contains(&needed) {
            println!("Found pairs: {} {}. Product: {}", number, needed,number*needed);
            break;
        }
    }
    
}