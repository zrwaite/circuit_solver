pub mod get_input {
	use crate::terminal_functions::terminal_functions as tf;
	use crate::parse::parse as parse;
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
	pub fn line() -> String{
		let mut line = String::new();
		io::stdin()
			.read_line(&mut line)
			.expect("Failed to read line");
		return line;
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
		return parse::line_to_vec(colour_line);
	}
}