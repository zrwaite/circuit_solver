use crate::commands::commands as commands;
use crate::circuit::circuit as circuit;
use crate::get_input::get_input as get_input;
use crate::terminal_functions::terminal_functions as tf;
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
			// let new_voltage = commands::get_new_element("voltage");
			// if new_voltage.2 {
			// 	circuit.add_voltage_source(new_voltage.0, new_voltage.1);
			// }
		}
		"add_c" => {
			// let new_current = commands::get_new_element("current");
			// if new_current.2 {
			// 	circuit.add_current_source(new_current.0, new_current.1);
			// }
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




pub fn add_r(command_strings: Vec<String>, circuit: &mut circuit::Circuit) {
	let flags = commands::get_flags(command_strings);
	if !flags.2 { //Failed to get flags from command line
		return;
	}
	let mut success = true;
	let mut r_name = ("".to_string(), false);
	let mut r_value = (0.0, false);
	for (key, value) in flags.0 {
		match key.to_lowercase().as_str() {
			"n"=> {
				r_name = (value, true);
			}
			"v"=> {
				let r_value_f: f32;
				r_value_f = match value.trim().parse() {
					Ok(num) => num,
					Err(_) => {
						tf::println_fail(&format!("Invalid value -v {}", value));
						success = false;
						continue;
					}
				};
				r_value = (r_value_f, true);
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
		return;
	} 
	if !r_name.1 {
		r_name.0 = get_input::name("Enter resistor name: ".to_string());
		r_name.1 = true;
	}
	if !r_value.1 {
		r_value.0 = get_input::value("Enter resistor value: ".to_string());
		r_value.1 = true;
	}
	println!("{}", "Added!".green());
	circuit.add_resistor(r_name.0, r_value);
}

pub fn print_help() {
	tf::println_info("add_r: begins process to add resistor to circuit");
	tf::println_info("add_v: begins process to add voltage to circuit");
	tf::println_info("add_c: begins process to add current to circuit");
	tf::println_info("help: prints different command options");
	tf::println_info("print: prints all elements in circuit");
	tf::println_info("q: quit program");
}
