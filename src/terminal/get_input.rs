pub mod get_input {
	use crate::terminal_functions;
	use terminal_functions::terminal_functions as tf;
	use std::io;
	use colored::*;
	pub fn name(prompt:String) -> String {
		let mut name = String::new();
		tf::print(&prompt);
		io::stdin()
			.read_line(&mut name)
			.expect("Failed to read line");
		return name.trim().to_string();
	}
	
	pub fn value(prompt:String) -> f32 {
		let value: f32;
		loop {
			let mut value_string = String::new();
			tf::print(&prompt);
			io::stdin()
				.read_line(&mut value_string)
				.expect("Failed to read line");
			value = match value_string.trim().parse() {
				Ok(num) => num,
				Err(_) => {
					println!("{}", "invalid value, try again".red());
					continue;
				}
			};
			break;
		}
		return value;
	}
	
	pub fn read_colours() -> Vec<String>{
		tf::print("Enter colours in one line: ");
		let mut colour_line = String::new();
		io::stdin()
			.read_line(&mut colour_line)
			.expect("Failed to read line");
		
		let colour_strings = colour_line.split_whitespace();
		let colour_slices: Vec<&str> = colour_strings.collect();
		let mut colours =  Vec::new();
		for colour in colour_slices.iter() {
			colours.push(colour.to_string());
		}
		return colours;
	}
}