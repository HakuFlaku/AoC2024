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

//	let problem_1 = process_puzzle_problem_1(rules, data);
//
//	println!("Solution for problem 1 is: {problem_1}")

	let problem_2 = process_puzzle_problem_2(&rules, data);

	println!("Solution for problem 2 is: {problem_2}");
}

fn process_puzzle_problem_2(rules: &Vec<Rule>, manuals: Vec<Vec<u32>>) -> u32 {
	let mut running_total = 0;

	for manual in filter_for_invalid_manuals(rules, manuals) {
		debug!("Analyzing the manual to find a valid format or not {:?}.", &manual);
		let mut fixed_manual = attempt_fix_to_manual(rules, &manual);
		
		while validate_manual(rules, &fixed_manual) == 0 {
			fixed_manual = attempt_fix_to_manual(rules, &fixed_manual);
		}
		debug!("Finished re-ordering manual, final ordering is: {:?}.", &fixed_manual);

		running_total += *fixed_manual.index(fixed_manual.len()/2);
		debug!("**** Our running total is now {:?}.", &running_total);
	}

	running_total
}

fn filter_for_invalid_manuals(rules: &Vec<Rule>, manuals: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
	let mut invalid_manuals = vec![];

	for manual in manuals {
		debug!("Checking if the manual {:?} is valid.", &manual);

		if validate_manual(rules, &manual) == 0 {
			invalid_manuals.push(manual);
		}
	}

	invalid_manuals
}

fn attempt_fix_to_manual(rules: &Vec<Rule>, manual: &Vec<u32>) -> Vec<u32> {
	let mut failed_rule_index: Option<usize> = None;

	for i in 0..manual.len() - 1 {
		let page = manual.get(i).unwrap();

		let rules_for_page: Vec<u32> = rules::find_rules_for(&rules, *page)
			.iter()
			.map(|x| x.before)
			.collect();

		for remainder_page in &manual[i..manual.len()] {
			if rules_for_page.contains(remainder_page) {
				failed_rule_index = Some(i);
				break;
			}
		}

		if failed_rule_index.is_some() {
			break;
		}
	}

	let mut re_ordered_manual = manual.clone();
	if failed_rule_index.is_some() {
		debug!("The index of the first page that failed a rule is: {:?}.", &failed_rule_index.unwrap());
		let end_val = re_ordered_manual.remove(failed_rule_index.unwrap());
		re_ordered_manual.push(end_val);
	}
	debug!("The re-ordered manual is {:?}.", &re_ordered_manual);

	re_ordered_manual
}

fn process_puzzle_problem_1(rules: Vec<Rule>, data: Vec<Vec<u32>>) -> u32 {
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
