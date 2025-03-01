use std::ops::Index;
use std::{fs, io};

use log::debug;
use env_logger;

use self::rules::Rule;

mod rules;

fn main() {
	env_logger::init();
	let input  = read_file(&get_filename());

	let (rules, data) = extract_puzzle(input);

	let problem_1 = process_puzzle(rules, data);

	println!("Solution for problem 1 is: {problem_1}")
}

fn process_puzzle(rules: Vec<Rule>, data: Vec<Vec<u32>>) -> u32 {
	let mut running_total = 0;

	for manual in data {
		debug!("***** Starting the next manual. *****");

		let manual_val = validate_manual(&rules, &manual);
		debug!("Manual finished process returned value is: {}.", &manual_val);

		running_total += manual_val;

		debug!("Current total is: {}.", &running_total);
	}

	running_total
}

fn validate_manual(rules: &Vec<Rule>, manual: &Vec<u32>) -> u32 {
	let mut is_valid = true;

	let mut local_manual = manual.clone();
	while !local_manual.is_empty() && is_valid {
		debug!("Manual still has content to process.");

		let (head, tail) = local_manual.split_first().unwrap();
		debug!("Current value being tested is {} with the remainder being {:?}", &head, &tail);

		let rules_for_page: Vec<u32> = rules::find_rules_for(&rules, *head)
			.iter()
			.map(|rule| rule.before)
			.collect();
		debug!("The rules found for current head are {:?}", &rules_for_page);

		for page in tail {
			debug!("Checking if the page {} is in the list of rules.", &page);
			debug!("Does the rules contain this page? {}", rules_for_page.contains(&page));

			is_valid = !rules_for_page.contains(page);
			if !is_valid {
				break;
			}
		}
		debug!("Is this manual valid? {}", is_valid);

		local_manual = tail.to_vec();
	}

	if is_valid {
		*manual.index(manual.len()/2)
	}
	else {
		0
	}
}

fn extract_puzzle(input: Vec<String>) -> (Vec<Rule>, Vec<Vec<u32>>)
{
	let mut rules = vec![];
	let mut data = vec![];
	let mut found_break = false;

	for line in input {
		debug!("Working on the line '{}'", &line);

		if line.is_empty() {
			found_break = true;
			continue;
		}

		if !found_break {
			rules.push(Rule::from(&line))
		}
		else {
			data.push(line.split(',')
				.map(|val| val.parse::<u32>().unwrap())
				.collect()
			)
		}
	}

	(rules, data)
}

fn get_filename() -> String
{
	let mut input = String::new();
	io::stdin()
		.read_line(&mut input)
		.expect("Could not read input.");
	input.trim().to_string()
}

fn read_file(filename: &str) -> Vec<String>
{
	fs::read_to_string(format!("./src/{filename}.txt"))
		.expect("Could not read file.")
		.lines()
		.map(|row| row.trim())
		.map(|row| row.to_string())
		.collect()
}
