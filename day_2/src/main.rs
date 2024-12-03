use std::fs;

fn main() 
{
    let input_file = String::from("input");
    let formatted_problem_input = format_puzzle(input_file);
    
//    let safe_reports = count_safe_reports(formatted_problem_input);
    let safe_reports = dampen_reports(formatted_problem_input);

    println!("The number of valid reports are {}", safe_reports);
}

fn format_puzzle(filename: String) -> Vec<Vec<i32>>
{
    let file_contents = fs::read_to_string(format!("./src/{}.txt", filename))
        .expect("Failed to read file.");

    let mut formatted_data = Vec::new();
    for line in file_contents.lines()
    {
        let mut report = Vec::new();
        for number in line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()
        {
            report.push(number);
        }
        formatted_data.push(report);
    }

    return formatted_data;
}

fn count_safe_reports(reports: Vec<Vec<i32>>) -> i32
{
    let mut count = 0;

    for report in reports
    { 
        if valid_report(&report)
        {
            count += 1;
        }
    }

    return count;
}

fn valid_report(report: &Vec<i32>) -> bool
{
    //dbg!("Processing report: {}", &report);
    let mut valid = true;
    let is_ascending = (report[0] - report[1]) < 0;
    let mut prior_value = -1;

    for value in report
    {
        let value = *value;
        if prior_value != -1
        {
            if is_ascending && prior_value > value
            {
                //dbg!("Broke rule that report should be always ascending.");
                valid = false;
                break;
            }
            else if !is_ascending && prior_value < value
            {
                //dbg!("Broke rule that report should be always descending.");
                valid = false;
                break;
            }

            let difference = (prior_value - value).abs();
            //dbg!("Calculated difference is {}", difference);
            if difference == 0 || difference > 3
            {
                //dbg!("Broke rule that report must make steps of at least 1 but no more than 3.");
                valid = false;
                break;
            }
        }
        prior_value = value;
    }

    return valid;
}

fn dampen_reports(reports: Vec<Vec<i32>>) -> i32
{
    let mut safe_report_count = 0;

    for report in reports
    {
        if valid_report(&report)
        {
            safe_report_count += 1;
            continue;
        }

        let report_variants = generate_report_variants(&report);

        for variant in report_variants
        {
            if valid_report(&variant)
            {
                safe_report_count += 1;
                break;
            }
        }
    }

    return safe_report_count;
}

fn generate_report_variants(to_varry: &Vec<i32>) -> Vec<Vec<i32>>
{
    let mut variations = Vec::new();

    for i in 0..to_varry.len()
    {
        let mut variation = Vec::new();

        for (position, element) in to_varry.iter().enumerate()
        {
            let element = *element;

            if position != i
            {
                variation.push(element);
            }
        }

        variations.push(variation);
    }

    return variations;
}
