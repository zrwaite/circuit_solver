pub mod voltage_source {
	use crate::element_values::element_values as element_values;
	pub struct VoltageSource {
		pub values: element_values::ElementValues,
		pub name: String
	}
	
	impl VoltageSource {
		pub fn new(
			voltage: (f32, bool),
			name: String
		) -> VoltageSource{
			//Constructor
			let new_voltage_source = VoltageSource{
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
	}
}