pub mod terminal {
	use crate::commands::commands as commands;
	use crate::circuit::circuit as circuit;
	use crate::circuit_node::circuit_node as node;
	use crate::get_input::get_input as get_input;
	use crate::terminal_functions::terminal_functions as tf;
	use std::collections::HashMap;
	
	use colored::*;
	
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
				print_help();
			}
			"add_r" => {
				add_r(command_strings[1..].to_vec(), circuit);
			}
			"add_v" => {
				add_v(command_strings[1..].to_vec(), circuit);
			}
			"add_c" => {
				add_c(command_strings[1..].to_vec(), circuit);
			}
			"add_n" => {
				add_n(command_strings[1..].to_vec(), circuit);
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
	
	pub fn add_node(
		flags: (HashMap<String, String>, Vec<String>, bool), 
		resistors: Vec<String>, 
		current_sources: Vec<String>,
		voltage_sources: Vec<String>) -> (node::Node, bool) {
		let new_node = node::Node::new();
		let mut success = true;
		for (key, value) in flags.0 {
			match key.to_lowercase().as_str() {
				"r"=> {
					
				}
				"v"=> {

				}
				_ => {
					tf::println_fail(&format!("Invalid flag -{}", key));
				}
			}
		}
		for double_flag in flags.1.iter() {
			match double_flag.to_lowercase().as_str() {
				_ => {
					tf::println_fail(&format!("Invalid flag --{}", double_flag));
				}
			}
		}
		(new_node, success)
	}

	
	pub fn add_elem(elem_type:&str, flags: (HashMap<String, String>, Vec<String>, bool)) -> (String, f32, bool) {
		let mut success = true;
		let mut r_name = ("".to_string(), false);
		let mut r_value = (0.0, false);
		for (key, value) in flags.0 {
			match key.to_lowercase().as_str() {
				"n"=> {
					r_name = (value, true);
				}
				"v"=> {
					let fail_message = &format!("Invalid value -v {}", value);
					r_value = commands::parse_f32(value, fail_message);
					if !r_value.1 {success = false};
				}
				_ => {
					tf::println_fail(&format!("Invalid flag -{}", key));
				}
			}
		}
		for double_flag in flags.1.iter() {
			match double_flag.to_lowercase().as_str() {
				_ => {
					tf::println_fail(&format!("Invalid flag --{}", double_flag));
				}
			}
		}
		if !success {
			return ("".to_string(), 0.0, false);
		} 
		if !r_name.1 {
			r_name.0 = get_input::name(format!("Enter {} name: ", elem_type));
		}
		if !r_value.1 {
			r_value.0 = get_input::value(format!("Enter {} value: ", elem_type));
		}
		return (r_name.0, r_value.0, true)
	}
	
	pub fn add_r(command_strings: Vec<String>, circuit: &mut circuit::Circuit) {
		let flags = commands::get_flags(command_strings);
		//Failed to get flags from command line
		if !flags.2 {return;}
		let resistance = add_elem("resistor", flags);
		if !resistance.2 {return;}
		println!("{}", "Added!".green());
		circuit.add_resistor(resistance.0, (resistance.1, true));
	}
	pub fn add_c(command_strings: Vec<String>, circuit: &mut circuit::Circuit) {
		let flags = commands::get_flags(command_strings);
		//Failed to get flags from command line
		if !flags.2 {return;}
		let current_source = add_elem("current", flags);
		if !current_source.2 {return;}
		println!("{}", "Added!".green());
		circuit.add_current_source(current_source.0, (current_source.1, true));
	}
	pub fn add_v(command_strings: Vec<String>, circuit: &mut circuit::Circuit) {
		let flags = commands::get_flags(command_strings);
		//Failed to get flags from command line
		if !flags.2 {return;}
		let voltage_source = add_elem("voltage", flags);
		if !voltage_source.2 {return;}
		println!("{}", "Added!".green());
		circuit.add_voltage_source(voltage_source.0, (voltage_source.1, true));
	}
	pub fn add_n(command_strings: Vec<String>, circuit: &mut circuit::Circuit) {
		let flags = commands::get_flags(command_strings);
		//Failed to get flags from command line
		if !flags.2 {return;}
		let node = add_node(flags, circuit.get_resistor_names(), circuit.get_current_source_names(), circuit.get_voltage_source_names());
		if !node.1 {return;}
		circuit.add_node();
		println!("{}", "Added!".green());
	}
	
	pub fn print_help() {
		tf::println_info("add_r: Add resistor to circuit. Flags: -n: Name(String) -v: Value(f32)");
		tf::println_info("add_v: Add voltage to circuit. Flags: -n: Name(String) -v: Value(f32)");
		tf::println_info("add_c: Add current to circuit. Flags: -n: Name(String) -v: Value(f32)");
		tf::println_info("help: Prints different command options");
		tf::println_info("print: Prints all elements in circuit");
		tf::println_info("q: Quit program");
	}
}
