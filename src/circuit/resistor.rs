pub mod resistor {
	use crate::element_values::element_values as element_values;
	
	pub struct Resistor {
		pub values: element_values::ElementValues,
		pub name: String
	}
	
	impl Resistor {
		/* Getters */
		pub fn print(&self) {
			println!("{} values:", self.name);
			self.values.print();
		}
		pub fn new(
			resistance: (f32, bool),
			name: String
		) -> Resistor{
			//Constructor
			let new_resistor = Resistor{
				values: element_values::ElementValues::new(
					(0.0, false),
					resistance, 
					(0.0, false),
					(0.0, false),
				),
				name
			};
			return new_resistor;
		}
	}
	
	pub fn get_value(value_type: String, colour:String) -> i32 {
		if !(value_type.eq("num") || value_type.eq("multiply")) {
			panic!("Invalid value_type in get_value");
		}
		let value;
		match colour.to_lowercase().as_str() {
			"black" => {value = 0;}
			"brown" => {value = 1;}
			"red" => {value = 2;}
			"orange" => {value = 3;}
			"yellow" => {value = 4;}
			"green" => {value = 5;}
			"blue" => {value = 6;}
			"violet"|"purple"|"pink" => {value = 7;}
			"grey"|"gray" => {value = 8;}
			"white"=> {value = 9;}
			"gold"=> {
				if value_type.eq("num") {
					panic!("Gold has no associated num");
				}
				value = -1;
			}
			"silver"=> {
				if value_type.eq("num") {
					panic!("Silver has no associated num");
				}
				value = -2;
			}
			_ => {
				panic!("Invalid colour!");
			}
		}
		return value;
	}
	
	pub fn get_resistance_from_colours(colours: Vec<String>) -> f32 {
		let num_colours = colours.len();
		if num_colours<3 {
			panic!("Not enough colours");
		} else if num_colours>6 {
			panic!("Too many colours");
		} else {
			let mut multiplier_index:u8 = 2;
			if num_colours == 6 || num_colours == 5 {
				multiplier_index = 3;
			}
			let multiplier_power = get_value("num".to_string(), colours[multiplier_index as usize].to_string());
			let multiplier:f32 = (10_f32).powi(multiplier_power);
			let mut resistance = 0.0;
			let mut i:u8 = (multiplier_index).into();
			while i>0 {
				i-=1;
				let colour_value = get_value("num".to_string(), colours[i as usize].to_string());
				let value:f32 = colour_value as f32;
				let value_multiplier = multiplier*10_f32.powi((multiplier_index-1-i).into());
				resistance += value * value_multiplier;
			}
			return resistance;
		}
	}
}