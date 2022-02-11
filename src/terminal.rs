use std::io;
pub fn read_colours() -> Vec<String>{
	println!("Enter colours in one line:");
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