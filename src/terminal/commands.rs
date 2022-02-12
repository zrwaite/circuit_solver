#[path = "./get_input.rs"] mod get_input;
use get_input as get;
use colored::*;

pub fn get_new_element(element_type: &str) -> (String, (f32, bool), bool) {
	let name_prompt = format!("Enter {} name: ", element_type);
	let value_prompt = format!("Enter {} value: ", element_type);
	let name = get::name(name_prompt);
	let value = get::value(value_prompt);
	println!("{}", "Added!".green());
	return (name.trim().to_string(), value, true);
}