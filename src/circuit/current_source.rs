mod element_values;

pub fn create_current_source(
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

pub struct CurrentSource {
	values: element_values::ElementValues,
	name: String
}

impl CurrentSource {
	/* Getters */
	pub fn get_values(&self) -> &element_values::ElementValues {&self.values}
}