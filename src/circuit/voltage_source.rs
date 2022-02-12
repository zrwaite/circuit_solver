mod element_values;

pub struct VoltageSource {
	values: element_values::ElementValues,
	name: String
}

impl VoltageSource {
	pub fn new(
		voltage: (f32, bool),
		name: String
	) -> VoltageSource{
		//Constructor
		let mut new_voltage_source = VoltageSource{
			values: element_values::ElementValues::new(
				voltage, 
				(0.0, false),
				(0.0, false),
				(0.0, false),
			),
			name
		};
		return new_voltage_source;
	}
	/* Getters */
	pub fn get_values(&self) -> &element_values::ElementValues {&self.values}
	pub fn get_name(&self) -> &str {&self.name}
}