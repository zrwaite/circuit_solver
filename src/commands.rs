pub mod commands {
	use crate::terminal_functions;
	use terminal_functions::terminal_functions as tf;
	use std::collections::HashMap;
	
	pub fn is_single_flag(arg: String) -> bool {
		if arg.len() < 2 {return false;}
		else if arg[0..1].to_string().eq("-") {
			if arg.len()> 2 && arg[1..2].to_string().eq("-") {
				return false;
			}
			return true;
		}
		false
	}
	
	pub fn is_double_flag(arg: String) -> bool {
		if arg.len() < 3 {return false;}
		else if arg[0..2].to_string().eq("--") {return true;}
		false
	}
	
	pub fn get_flags(command_strings: Vec<String>) -> (HashMap<String, String>, Vec<String>, bool) {
		let mut success = true;
		let mut single_flags:HashMap<String, String> = HashMap::new();
		let mut double_flags:Vec<String> = Vec::new();
		let mut single_flag = ("", false);
		for command_string in command_strings.iter() {
			if is_single_flag(command_string.to_string()) {
				if single_flag.1 && !single_flags.contains_key(single_flag.0) {
					// New flag being declared while previous flag has no value
					tf::println_fail(&format!("Missing params for flag -{}", single_flag.0));
					success = false;
				}
				//Set new flag
				single_flag = (&command_string[1..], true);
			} else if is_double_flag(command_string.to_string()) {
				if single_flag.1 && !single_flags.contains_key(single_flag.0) {
					// New flag being declared while previous flag has no value
					tf::println_fail(&format!("Missing params for flag -{}", single_flag.0));
					success = false;
				}
				single_flag.1 = false;
				//Add double flag to list
				double_flags.push(command_string[2..].to_string());
			} else if single_flag.1 {
				if !single_flags.contains_key(single_flag.0) {
					single_flags.insert(single_flag.0.to_string(), command_string.to_string());
				} else {
					single_flags.insert(single_flag.0.to_string(), format!("{} {}", single_flags[single_flag.0], command_string));
				}
			} else {
				tf::println_fail(&format!("Invalid argument '{}'", command_string));
				success = false;
			} 
		}
		(single_flags, double_flags, success)
	}
	pub fn parse_f32(string_value: String, fail_message: &str) -> (f32, bool) {
		let f32_value: f32;
		f32_value = match string_value.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				tf::println_fail(fail_message);
				return (0.0, false);
			}
		};
		(f32_value, true)
	}	
}