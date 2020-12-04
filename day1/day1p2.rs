use std::fs;
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
    let numbers_str  = contents.lines();
    let numbers: HashSet<i32> = numbers_str.map(|s| s.parse().unwrap()).collect();
    let mut number_pair_sums: Vec<i32> = Vec::<i32>::new();
    let mut number_pair_products: Vec<i32> = Vec::<i32>::new();
    let target: i32 = 2020;

    for i in &numbers{
        for j in &numbers{
            if i == j{
                continue;
            }
            number_pair_sums.push(i+j);
            number_pair_products.push(i*j);
        }
    }

    for i in 0..number_pair_sums.len() {
        let needed: i32 = target - number_pair_sums[i];
        if numbers.contains(&needed) {
            println!("Product: {}", number_pair_products[i]*needed);
            break;
        }
    }
   
}