use std::io;
#[path = "./circuit.rs"] mod circuit;


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
pub fn build_circuit() -> circuit::Circuit {
	let mut new_circuit = circuit::Circuit::new();
	println!("Enter commands here. \"help\" for more info");
	loop {
		let mut command = String::new();
		io::stdin()
			.read_line(&mut command)
			.expect("Failed to read line");
		match command.trim().to_lowercase().as_str() {
			"help" => {
				print_commands();
			}
			"add_resistor" => {
				let new_resistor = get_new_resistor();
				if new_resistor.2 {
					new_circuit.add_resistor(new_resistor.0, new_resistor.1);
				}
			}
			"print_elements" => {
				let elements = new_circuit.get_resistors();
				for item in elements.iter() {
					item.print()
				}
			}
			"q"=> {
				break;
			}
			_ => {
				println!("Invalid Command. q to quit");
			}
		}
	}
    


	return new_circuit;
}

pub fn print_commands() {
	println!("add_resistor: begins process to add resistor to curcuit");
	println!("help: prints different command options");
	println!("print_elements: prints all elements in circuit");
	println!("q: quit program");
}

pub fn get_new_resistor() -> (String, (f32, bool), bool) {
	let mut name = String::new();
	let mut value_string = String::new();
	let mut value: f32;

	println!("Enter resistor name: ");
	io::stdin()
		.read_line(&mut name)
		.expect("Failed to read line");
	loop {
		println!("Enter resistor value: ");
		io::stdin()
			.read_line(&mut value_string)
			.expect("Failed to read line");
		value = match value_string.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("invalid value, try again");
				continue;
			}
		};
		break;
	}
	println!("Added!");
	return (name.trim().to_string(), (value, true), true);
}