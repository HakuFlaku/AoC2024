pub struct Rule {
	pub before: u32,
	pub key: u32
}

impl Rule {
	pub fn new(before: u32, key: u32) -> Rule {
		return Rule{ before, key }
	}

	pub fn from(from: &String) -> Rule {
		let vals: Vec<&str> = from.split('|').collect();
		let before = vals[0].parse::<u32>().unwrap();
		let key = vals[1].parse::<u32>().unwrap();

		Rule{ before, key }
	}
}

pub fn find_rules_for(rules: &Vec<Rule>, key: u32) -> Vec<&Rule>
{
	let mut matching_rules = vec![];

	for rule in rules {
		if rule.key == key {
			matching_rules.push(rule)
		}
	}

	return matching_rules;
}
