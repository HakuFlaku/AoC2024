use std::{fs, io};

struct Coordinate
	{
		x: usize,
		y: usize
	}

impl Coordinate
{
	fn new(x: usize, y: usize) -> Coordinate
	{
		return Coordinate
			{
			x,
			y
		}
	}
}

fn main() 
{
	let puzzle_data = read_file(&get_filename());

	//    let character_start_positions = find_chars(&puzzle_data, 'X');

	let character_start_positions = find_chars(&puzzle_data, 'A');

	//    let found_count = find_xmas_in(&character_start_positions, &puzzle_data);

	let found_count = find_x_max_in(&character_start_positions, &puzzle_data);

	println!("The total number of XMAS words in the puzzle is {}", &found_count);
}

fn find_x_max_in(coords: &Vec<Coordinate>, puzzle: &Vec<Vec<char>>) -> i32
{ 
	let mut found_count = 0;
	let y_min: usize = 1;
	let y_max = puzzle.len() - 2;
	let x_min: usize = 1;
	let x_max = puzzle[0].len() - 2;   

	for coord in coords
	{
		if coord.y >= y_min && coord.x >= x_min && 
		coord.y <= y_max && coord.x <= x_max
		{
			if (puzzle[coord.y - 1][coord.x - 1] == puzzle[coord.y + 1][coord.x - 1] && puzzle[coord.y + 1][coord.x - 1] == 'M') &&
			(puzzle[coord.y - 1][coord.x + 1] == puzzle[coord.y + 1][coord.x + 1] && puzzle[coord.y + 1][coord.x + 1] == 'S')
			{
				found_count += 1;
			}

			if (puzzle[coord.y - 1][coord.x - 1] == puzzle[coord.y + 1][coord.x - 1] && puzzle[coord.y + 1][coord.x - 1] == 'S') &&
			(puzzle[coord.y - 1][coord.x + 1] == puzzle[coord.y + 1][coord.x + 1] && puzzle[coord.y + 1][coord.x + 1] == 'M')
			{
				found_count += 1;
			}

			if (puzzle[coord.y + 1][coord.x - 1] == puzzle[coord.y + 1][coord.x + 1] && puzzle[coord.y + 1][coord.x + 1] == 'M') &&
			(puzzle[coord.y - 1][coord.x - 1] == puzzle[coord.y - 1][coord.x + 1] && puzzle[coord.y - 1][coord.x - 1] == 'S')
			{
				found_count += 1;
			}

			if (puzzle[coord.y + 1][coord.x - 1] == puzzle[coord.y + 1][coord.x + 1] && puzzle[coord.y + 1][coord.x + 1] == 'S') &&
			(puzzle[coord.y - 1][coord.x - 1] == puzzle[coord.y - 1][coord.x + 1] && puzzle[coord.y - 1][coord.x - 1] == 'M')
			{
				found_count += 1;
			}
		}
	}

	return found_count;
}

fn find_xmas_in(coords: &Vec<Coordinate>, puzzle: &Vec<Vec<char>>) -> i32
{
	let mut found_count = 0;
	let y_min: usize = 3;
	let y_max = puzzle.len() - 4;
	let x_min: usize = 3;
	let x_max = puzzle[0].len() - 4;

	for coord in coords
	{
		if coord.y >= y_min && coord.x >= x_min &&
		puzzle[coord.y - 1][coord.x - 1] == 'M' &&
		puzzle[coord.y - 2][coord.x - 2] == 'A' &&
		puzzle[coord.y - 3][coord.x - 3] == 'S'
		{
			found_count += 1;
		}

		if coord.y >= y_min && coord.x <= x_max &&
		puzzle[coord.y - 1][coord.x + 1] == 'M' &&
		puzzle[coord.y - 2][coord.x + 2] == 'A' &&
		puzzle[coord.y - 3][coord.x + 3] == 'S'
		{
			found_count += 1;
		}

		if coord.y >= y_min &&
		puzzle[coord.y - 1][coord.x] == 'M' &&
		puzzle[coord.y - 2][coord.x] == 'A' &&
		puzzle[coord.y - 3][coord.x] == 'S'
		{
			found_count += 1;
		}

		if coord.y <= y_max && coord.x >= x_min &&
		puzzle[coord.y + 1][coord.x - 1] == 'M' &&
		puzzle[coord.y + 2][coord.x - 2] == 'A' &&
		puzzle[coord.y + 3][coord.x - 3] == 'S'
		{
			found_count += 1;
		}

		if coord.y <= y_max && coord.x <= x_max &&
		puzzle[coord.y + 1][coord.x + 1] == 'M' &&
		puzzle[coord.y + 2][coord.x + 2] == 'A' &&
		puzzle[coord.y + 3][coord.x + 3] == 'S'
		{
			found_count += 1;
		}

		if coord.y <= y_max &&
		puzzle[coord.y + 1][coord.x] == 'M' &&
		puzzle[coord.y + 2][coord.x] == 'A' &&
		puzzle[coord.y + 3][coord.x] == 'S'
		{
			found_count += 1;
		}

		if coord.x >= x_min &&
		puzzle[coord.y][coord.x - 1] == 'M' &&
		puzzle[coord.y][coord.x - 2] == 'A' &&
		puzzle[coord.y][coord.x - 3] == 'S'
		{
			found_count += 1;
		}

		if coord.x <= x_max &&
		puzzle[coord.y][coord.x + 1] == 'M' &&
		puzzle[coord.y][coord.x + 2] == 'A' &&
		puzzle[coord.y][coord.x + 3] == 'S'
		{
			found_count += 1;
		}
	}

	return found_count;
}

fn get_filename() -> String
{
	let mut input = String::new();
	io::stdin()
		.read_line(&mut input)
		.expect("Could not read input.");
	input.trim().to_string()
}

fn read_file(filename: &str) -> Vec<Vec<char>>
{
	return fs::read_to_string(format!("./src/{filename}.txt"))
		.expect("Could not read file.")
		.lines()
		.map(|row| row.trim())
		.map(|row| row.to_string())
		.map(|row| row.chars().collect())
		.collect();
}

fn find_chars(grid: &Vec<Vec<char>>, to_be: char) -> Vec<Coordinate>
{
	let mut coordinates = Vec::new();

	for (y, row) in grid.iter().enumerate()
	{
		for (x, element) in row.iter().enumerate()
		{
			if *element == to_be
			{
				coordinates.push(Coordinate::new(x, y));
			}
		}
	}

	return coordinates;
}
