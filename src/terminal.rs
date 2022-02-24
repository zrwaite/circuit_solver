pub mod terminal {
	//add_n -r 
	use crate::parse::parse as parse;
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
			let command_line = get_input::line();
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
				print_circuit(command_strings[1..].to_vec(), circuit);
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
	pub fn print_circuit(command_strings: Vec<String>, circuit: &circuit::Circuit) {
		let flags = parse::get_flags(command_strings);
		circuit.print_elements();
	}
	//add_n -r R3 R1 R2 -v V1+ V2- -c 
	pub fn add_elem_to_node(key:String, value:String, new_node:&mut node::Node, 
		resistor_names: &Vec<String>, current_source_names: &Vec<String>, 
		voltage_source_names: &Vec<String> ) -> bool {
		let mut success = false;
		match key.to_lowercase().as_str() {
			"r"=> {
				if resistor_names.contains(&value.clone()){
					let index_found = parse::get_index(&resistor_names, &value);
					if index_found.1 {
						new_node.add_resistor(index_found.0);
					} else {
						panic!("Array contains element but element not found");
					}
				} else {
					tf::println_fail(&format!("Resistor not found: {}", value));
					success = false;
				}
			}
			"v"=> {
				let sign = value[value.len()-1..value.len()].to_string();
				let name = value[0..value.len()-1].to_string();
				if !(sign.eq("-") || sign.eq("+")) {
					tf::println_fail(&format!("Voltage Source direction not properly specified: {}", value));
					return false;
				}
				if voltage_source_names.contains(&name.clone()){
					let index_found = parse::get_index(&voltage_source_names, &name);
					if index_found.1 {
						new_node.add_voltage_source(index_found.0, sign.eq("+"));
					} else {
						panic!("Array contains element but element not found");
					}
				} else {
					tf::println_fail(&format!("Voltage Source not found: {}", name));
					success = false;
				}
			}
			"c"=> {
				let sign = value[value.len()-1..value.len()].to_string();
				let name = value[0..value.len()-1].to_string();
				if !(sign.eq("-") || sign.eq("+")) {
					tf::println_fail(&format!("Current Source direction not properly specified: {}", value));
					return false;
				}
				if current_source_names.contains(&name.clone()){
					let index_found = parse::get_index(&current_source_names, &name);
					if index_found.1 {
						new_node.add_current_source(index_found.0, sign.eq("+"));
					} else {
						panic!("Array contains element but element not found");
					}
				} else {
					tf::println_fail(&format!("Current Source not found: {}", name));
					success = false;
				}
			}
			_ => {
				tf::println_fail(&format!("Invalid flag -{}", key));
				success = false;
			}
		}
		success
	}
	pub fn add_more_connections(
		new_node: &mut node::Node, 
		resistor_names: &Vec<String>,
		current_source_names: &Vec<String>,
		voltage_source_names: &Vec<String>) -> i32{
		let mut added_connections = 0;
		let resistor_list = get_input::name("Enter resistor names: ".to_string());
		for value in parse::line_to_vec(resistor_list).iter() {
			let added = add_elem_to_node("r".to_string(), value.to_string(), new_node, &resistor_names, &current_source_names, &voltage_source_names);
			if added {added_connections += 1}
		}
		let voltage_source_list = get_input::name("Enter voltage source names: ".to_string());
		for value in parse::line_to_vec(voltage_source_list).iter() {
			let added = add_elem_to_node("v".to_string(), value.to_string(), new_node, &resistor_names, &current_source_names, &voltage_source_names);
			if added {added_connections += 1}
		}
		let current_source_list = get_input::name("Enter current source names: ".to_string());
		for value in parse::line_to_vec(current_source_list).iter() {
			let added = add_elem_to_node("c".to_string(), value.to_string(), new_node, &resistor_names, &current_source_names, &voltage_source_names);
			if added {added_connections += 1}
		}
		added_connections
	}
	pub fn add_node(
		flags: (HashMap<String, String>, Vec<String>, bool), voltage_source_names: Vec<String>, resistor_names: Vec<String>, current_source_names: Vec<String>) -> (node::Node, bool) {
		let mut new_node = node::Node::new();
		let mut num_connections = 0;
		let mut success = true;
		for (key, values) in flags.0 {
			for value in parse::line_to_vec(values).iter(){
				let added = add_elem_to_node(key.clone(), value.to_string(), &mut new_node, &resistor_names, &current_source_names, &voltage_source_names);
				if added {num_connections += 1}
			}
		}
		for double_flag in flags.1.iter() {
			match double_flag.to_lowercase().as_str() {
				_ => {
					tf::println_fail(&format!("Invalid flag --{}", double_flag));
					success = false;
				}
			}
		}
		let mut looped = false;
		loop {
			let mut add_more = false;
			if num_connections < 2{
				if num_connections == 0 && looped{
					tf::println_info("At least 2 connections required");
					let add_more_input = get_input::name("Cancel? [y]/[n]: ".to_string());
					if add_more_input.eq("y") {
						success = false;
						break;
					}
				}
				add_more = true;
			} else {
				let add_more_input = get_input::name("Add more elements? [y]/[n]: ".to_string());
				if add_more_input.eq("y") {add_more = true}
			}
			if add_more {
				num_connections+=add_more_connections(&mut new_node, &resistor_names, &current_source_names, &voltage_source_names);
			} else {
				break;
			}
			looped = true;
		}
		(new_node, success)
	}

	pub fn add_elem(elem_type:&str, flags: (HashMap<String, String>, Vec<String>, bool), used_names:Vec<String>) -> (String, f32, bool) {
		let mut success = true;
		let mut r_name = ("".to_string(), false);
		let mut r_value = (0.0, false);
		for (key, value) in flags.0 {
			match key.to_lowercase().as_str() {
				"n"=> {
					r_name = (value.clone(), true);
					if used_names.contains(&value){
						r_name.1 = false;
						tf::println_fail("Element name already in use");
						success = false;
					}
				}
				"v"=> {
					let fail_message = &format!("Invalid value -v: {}", value);
					r_value = parse::parse_f32(value, fail_message);
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
		let flags = parse::get_flags(command_strings);
		//Failed to get flags from command line
		if !flags.2 {return;}
		let resistance = add_elem("resistor", flags, circuit.get_element_names());
		if !resistance.2 {return;}
		println!("{}", "Added!".green());
		circuit.add_resistor(resistance.0, (resistance.1, true));
	}
	pub fn add_c(command_strings: Vec<String>, circuit: &mut circuit::Circuit) {
		let flags = parse::get_flags(command_strings);
		//Failed to get flags from command line
		if !flags.2 {return;}
		let current_source = add_elem("current", flags, circuit.get_element_names());
		if !current_source.2 {return;}
		println!("{}", "Added!".green());
		circuit.add_current_source(current_source.0, (current_source.1, true));
	}
	pub fn add_v(command_strings: Vec<String>, circuit: &mut circuit::Circuit) {
		let flags = parse::get_flags(command_strings);
		//Failed to get flags from command line
		if !flags.2 {return;}
		let voltage_source = add_elem("voltage", flags, circuit.get_element_names());
		if !voltage_source.2 {return;}
		println!("{}", "Added!".green());
		circuit.add_voltage_source(voltage_source.0, (voltage_source.1, true));
	}
	pub fn add_n(command_strings: Vec<String>, circuit: &mut circuit::Circuit) {
		let flags = parse::get_flags(command_strings);
		//Failed to get flags from command line
		if !flags.2 {return;}
		let node = add_node(flags, circuit.get_voltage_source_names(), circuit.get_resistor_names(), circuit.get_current_source_names());
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
