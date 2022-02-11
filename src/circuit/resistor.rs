mod element_values;

pub fn create_resistor(
	resistance: (f32, bool),
	name: String
) -> Resistor{
	//Constructor
	let mut new_resistor = Resistor{
		values: element_values::create_element_values(
			(0.0, false),
			resistance, 
			(0.0, false),
			(0.0, false),
		),
		name
    };
	return new_resistor;
}

pub struct Resistor {
	values: element_values::ElementValues,
	name: String
}

impl Resistor {
	/* Getters */
	pub fn get_values(&self) -> &element_values::ElementValues {&self.values}
}

pub fn get_value(value_type: String, colour:String) -> i32 {
	if !(value_type.eq("num") || value_type.eq("multiply")) {
		panic!("Invalid value_type in get_value");
	}
	let mut value = 0;
	match colour.as_str() {
		"black"|"Black" => {value = 0;}
		"brown"|"Brown" => {value = 1;}
		"red"|"Red" => {value = 2;}
		"orange"|"Orange" => {value = 3;}
		"yellow"|"Yellow" => {value = 4;}
		"green"|"Green" => {value = 5;}
		"blue"|"Blue" => {value = 6;}
		"violet"|"Violet"|"purple"|"Purple"|"pink"|"Pink" => {value = 7;}
		"grey"|"Grey"|"gray"|"Gray" => {value = 8;}
		"white"|"White"=> {value = 9;}
		"gold"|"Gold"=> {
			if value_type.eq("num") {
				panic!("Gold has no associated num");
			}
			value = -1;
		}
		"silver"|"Silver"=> {
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