pub mod terminal_functions {
	use std::io;
	use std::io::Write;
	use colored::*;
	pub fn print(prompt:&str) {
		print!("{}", prompt.cyan());
		io::stdout().flush().unwrap();
	}
	pub fn println_info(prompt: &str) {
		println!("{}", prompt.italic().yellow());
	}
	pub fn println_header(prompt: &str) {
		println!("{}", prompt.bold().cyan());
	}
	pub fn println_fail(prompt: &str) {
		println!("{}", prompt.red());
	}
	pub fn print_prefix(prompt: &str) {
		print!("{}", prompt.bold());
	}
}	
