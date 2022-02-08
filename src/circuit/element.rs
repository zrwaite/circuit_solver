pub fn create_element() -> Element{
	let new_element = Element{
		voltage_drop: 0.0,
		resistance: 0.0,
		current: 0.0
    };
	return new_element;
}
pub struct Element {
	voltage_drop: f32,
	resistance: f32,
	current: f32
}

impl Element {
	pub fn get_voltage_drop(&self) -> f32 {
		return self.voltage_drop;
	}
	pub fn get_resistance(&self) -> f32 {
		return self.resistance;
	}
	pub fn get_current(&self) -> f32 {
		return self.current;
	}
}