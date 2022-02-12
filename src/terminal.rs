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
		let mut command_line = String::new();
		io::stdin()
			.read_line(&mut command_line)
			.expect("Failed to read line");

		let command_slices: Vec<&str> = command_line.split_whitespace().collect();
		let mut command_strings =  Vec::new();
		for command in command_slices.iter() {
			command_strings.push(command.to_string());
		}

		match command_strings[0].trim().to_lowercase().as_str() {
			"help" => {
				print_commands();
			}
			"add_resistor" => {
				let new_resistor = get_new_element("resistor");
				if new_resistor.2 {
					new_circuit.add_resistor(new_resistor.0, new_resistor.1);
				}
			}
			"add_voltage" => {
				let new_voltage = get_new_element("voltage");
				if new_voltage.2 {
					new_circuit.add_voltage_source(new_voltage.0, new_voltage.1);
				}
			}
			"add_current" => {
				let new_current = get_new_element("current");
				if new_current.2 {
					new_circuit.add_current_source(new_current.0, new_current.1);
				}
			}
			"print_elements" => {
				new_circuit.print_elements();
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
	println!("add_voltage: begins process to add voltage to curcuit");
	println!("add_current: begins process to add current to curcuit");
	println!("help: prints different command options");
	println!("print_elements: prints all elements in circuit");
	println!("q: quit program");
}

pub fn get_name(prompt:String) -> String {
	let mut name = String::new();
	println!("{}", prompt);
	io::stdin()
		.read_line(&mut name)
		.expect("Failed to read line");
	return name;
}

pub fn get_value(prompt:String) -> (f32, bool) {
	let mut value_string = String::new();
	let mut value: f32;
	loop {
		println!("{}", prompt);
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
	return (value, true);
}

pub fn get_new_element(element_type: &str) -> (String, (f32, bool), bool) {
	let name_prompt = format!("Enter {} name: ", element_type);
	let value_prompt = format!("Enter {} value: ", element_type);
	let name = get_name(name_prompt);
	let value = get_value(value_prompt);
	println!("Added!");
	return (name.trim().to_string(), value, true);
}


