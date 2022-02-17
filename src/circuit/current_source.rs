pub mod current_source {
	use crate::element_values::element_values as element_values;
	pub struct CurrentSource {
		pub values: element_values::ElementValues,
		pub name: String
	}
	
	impl CurrentSource {
		pub fn new(
			current: (f32, bool),
			name: String
		) -> CurrentSource{
			//Constructor
			let new_current_source = CurrentSource{
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
	}
}