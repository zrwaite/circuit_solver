mod element_values;

pub fn create_voltage_source(
	voltage: (f32, bool),
	name: String
) -> VoltageSource{
	//Constructor
	let mut new_voltage_source = VoltageSource{
		values: element_values::create_element_values(
			voltage, 
			(0.0, false),
			(0.0, false),
			(0.0, false),
		),
		name: name
    };
	return new_voltage_source;
}

pub struct VoltageSource {
	values: element_values::ElementValues,
	name: String
}

impl VoltageSource {
	/* Getters */
	pub fn get_values(&self) -> &element_values::ElementValues {&self.values}
}