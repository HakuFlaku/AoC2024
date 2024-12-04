use std::{fs, io};
use regex::Regex;

fn main() 
{
    let file_content = read_file();

    let formatted_puzzle_input = format_for_puzzle(file_content);
    
//    let total_valid_memory = extract_and_perform_calcs(formatted_puzzle_input);
    let total_valid_memory = programmatic_calculations(formatted_puzzle_input);

    println!("The total value from the invalid memory is {}.", &total_valid_memory);
}

fn read_file() -> String
{
    let mut filename = String::new();

    io::stdin()
        .read_line(&mut filename)
        .expect("Could not read user input.");
    let filename = filename.trim();

    let file_content = fs::read_to_string(format!("./src/{filename}.txt"))
        .expect("Could not read file.");

    return file_content;
}

fn format_for_puzzle(input: String) -> Vec<String>
{
    return input.lines()
        .map(|x| String::from(x))
        .collect();
}

fn programmatic_calculations(input: Vec<String>) -> i32
{
    let re = Regex::new(r"mul\(\d+,\d+\)|don't\(\)|do\(\)").unwrap();
    let mut sum = 0;
    let mut do_apply = true;
    
    for line in input
    {
        let matches: Vec<&str> = re.find_iter(&line)
            .map(|x| x.as_str())
            .map(|x| x.trim())
            .collect();

        for mat in matches
        {
            if mat.eq("don't()")
            {
                do_apply = false;
            }
            else if mat.eq("do()")
            {
                do_apply = true;
            }
            else if do_apply
            {
                sum += extract_numbers(mat).0 * extract_numbers(mat).1;
            }
        }
    }

    return sum;
}

fn extract_and_perform_calcs(input: Vec<String>) -> i32
{
    let regex = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let mut sum = 0;

    for line in input
    {
//        dbg!("Line in processing is {}", &line);

        let matches: Vec<&str> = regex.find_iter(&line)
            .map(|x| x.as_str())
            .map(|x| x.trim())
            .collect();

//        dbg!("Total matches to the regex are {}", &matches);

        let tuples: Vec<(i32, i32)> = matches.iter()
            .map(|x| extract_numbers(x))
            .collect();
        
//        dbg!("Total tuples found {}", &tuples);

        for tuple in tuples
        {
            sum += tuple.0 * tuple.1;
        }
    }

    return sum;
}

fn extract_numbers(from: &str) -> (i32, i32)
{
    let from = from.to_string();

    let reduced_data: Vec<&str> = from[4..from.len()-1]
        .split(',')
        .collect();

    let numbers: Vec<i32> = reduced_data
        .iter()
        .map(|x| x.parse::<i32>().expect("Could not extract number"))
        .collect();

    return (numbers[0], numbers[1]);
}
