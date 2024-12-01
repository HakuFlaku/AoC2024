use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main()
{
    // let dif = find_dif();
    // 
    // println!("Total difference: {}", dif);

    let similarity = find_similarity();

    println!("Total similarity: {}", similarity);
}

fn find_similarity() -> i32
{
    let mut source = Vec::new();
    let mut compare_map = HashMap::new();

    if let Ok(lines) = read_lines("./src/input.txt")
    {
        for line in lines.flatten()
        {
            let split_vals: Vec<&str> = line.split("   ").collect();
            let left = split_vals[0].parse::<i32>().unwrap();
            let right = split_vals[1].parse::<i32>().unwrap();

            source.push(left);
            let count = compare_map.entry(right).or_insert(0);
            *count += 1;
        }
    }

    let source = source;
    let compare_map = compare_map;
    
    let mut similarity_score = 0;
    for val in source
    {
        match compare_map.get(&val)
        {
            Some(multiplier) => similarity_score += val * multiplier,
            _ => (),
        }
    }

    return similarity_score;
}

#[allow(dead_code)]
fn find_dif() -> i32
{
    // read in the input
    // split by whitespace
    // filter elements into left list and right list
    // sort both lists
    // compare elements in order and grab the absolute
    // add total absolute differences
    // bada-bing bada-boom, that be da answer   let mut left_vals = Vec::new();

    let mut left_vals = Vec::new();
    let mut right_vals = Vec::new();

    if let Ok(lines) = read_lines("./src/input.txt")
    {
        for line in lines.flatten()
        {
            let split_vals: Vec<&str> = line.split("   ").collect();
            let left = split_vals[0].parse::<i32>().unwrap();
            let right = split_vals[1].parse::<i32>().unwrap();

            // println!("Left val {} right val {}", left, right);

            left_vals.push(left);
            right_vals.push(right);
        }
    }
    
    left_vals.sort();
    right_vals.sort();

    let left_vals = left_vals;
    let right_vals = right_vals;
    
    let mut total_dif = 0;
    for i in 0..left_vals.len()
    {
        total_dif += (left_vals[i] - right_vals[i]).abs();
    }
    let total_dif = total_dif;

    return total_dif;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
