#[path = "./circuit.rs"] mod circuit;
mod get_input;
mod terminal_functions;
use terminal_functions as tf;
mod commands;


pub fn build_circuit() -> circuit::Circuit {
	let mut new_circuit = circuit::Circuit::new();
	tf::println_header("Enter commands here. \"help\" for more info");
	loop {
		tf::print("> ");
		let command_line = tf::read_line();
		let command_slices: Vec<&str> = command_line.split_whitespace().collect();
		let mut command_strings =  Vec::new();
		for command in command_slices.iter() {
			command_strings.push(command.to_string());
		}
		if !run_commands(command_strings, &mut new_circuit) {break;}
	}
	return new_circuit;
}

pub fn run_commands(command_strings: Vec<String>, circuit: &mut circuit::Circuit) -> bool {
	match command_strings[0].trim().to_lowercase().as_str() {
		"help" => {
			print_commands();
		}
		"add_r" => {
			let new_resistor = commands::get_new_element("resistor");
			if new_resistor.2 {
				circuit.add_resistor(new_resistor.0, new_resistor.1);
			}
		}
		"add_v" => {
			let new_voltage = commands::get_new_element("voltage");
			if new_voltage.2 {
				circuit.add_voltage_source(new_voltage.0, new_voltage.1);
			}
		}
		"add_c" => {
			let new_current = commands::get_new_element("current");
			if new_current.2 {
				circuit.add_current_source(new_current.0, new_current.1);
			}
		}
		"print" => {
			circuit.print_elements();
		}
		"q"=> {
			return false;
		}
		_ => {
			tf::println_fail("Invalid Command. q to quit");
		}
	}
	return true;
}

pub fn print_commands() {
	tf::println_info("add_r: begins process to add resistor to curcuit");
	tf::println_info("add_v: begins process to add voltage to curcuit");
	tf::println_info("add_c: begins process to add current to curcuit");
	tf::println_info("help: prints different command options");
	tf::println_info("print: prints all elements in circuit");
	tf::println_info("q: quit program");
}


