mod element_values;

pub struct CurrentSource {
	values: element_values::ElementValues,
	name: String
}

impl CurrentSource {
	pub fn new(
		current: (f32, bool),
		name: String
	) -> CurrentSource{
		//Constructor
		let mut new_current_source = CurrentSource{
			values: element_values::ElementValues::new(
				(0.0, false),
				(0.0, false),
				current, 
				(0.0, false),
			),
			name
		};
		return new_current_source;
	}
	/* Getters */
	pub fn get_values(&self) -> &element_values::ElementValues {&self.values}
	pub fn get_name(&self) -> &str {&self.name}
}