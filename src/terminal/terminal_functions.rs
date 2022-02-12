use std::io;
use std::io::Write;
use colored::*;
pub fn read_line() -> String{
	let mut line = String::new();
	io::stdin()
		.read_line(&mut line)
		.expect("Failed to read line");
	return line;
}
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

