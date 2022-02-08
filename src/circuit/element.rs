pub fn create_element(
	voltage: (f32, bool),
	resistance: (f32, bool),
	current: (f32, bool)
) -> Element{
	let mut new_element = Element{
		voltage: voltage,
		resistance: resistance,
		current: current
    };
	new_element.try_ohm();
	return new_element;
}
pub struct Element {
	voltage: (f32, bool),
	resistance: (f32, bool),
	current: (f32, bool)
}

impl Element {
	pub fn get_voltage(&self) -> (f32, bool) {
		return self.voltage;
	}
	pub fn get_resistance(&self) -> (f32, bool) {
		return self.resistance;
	}
	pub fn get_current(&self) -> (f32, bool) {
		return self.current;
	}
	fn try_ohm(&mut self) {
		if self.voltage.1 && self.current.1 && self.resistance.1 {
			let error = self.voltage.0 - self.current.0 * self.resistance.0;
			if error.abs() < 0.1 {return;}
			panic!("Invalid value added to element");
		}
		if self.voltage.1 && self.current.1 && !self.resistance.1 {
			self.ohm_law_resistance()
		} else if !self.voltage.1 && self.current.1 && self.resistance.1 {
			self.ohm_law_voltage()
		} else if self.voltage.1 && !self.current.1 && self.resistance.1 {
			self.ohm_law_current()
		}
	}
	fn ohm_law_voltage(&mut self) {
		assert!(!self.voltage.1);
		assert!(self.current.1);
		assert!(self.resistance.1);
		self.voltage.0 = self.current.0 * self.resistance.0;
		self.voltage.1 = true;
	}
	fn ohm_law_current(&mut self) {
		assert!(!self.current.1);
		assert!(self.voltage.1);
		assert!(self.resistance.1);
		if self.resistance.0 == 0.0 {
			self.current.0 = f32::INFINITY;
		} else {
			self.current.0 = self.voltage.0 / self.resistance.0;
		}
		self.current.1 = true;
	}
	fn ohm_law_resistance(&mut self) {
		assert!(!self.resistance.1);
		assert!(self.voltage.1);
		assert!(self.current.1);
		if self.current.0 == 0.0 {
			self.resistance.0 = f32::INFINITY;
		} else {
			self.resistance.0 = self.voltage.0 / self.current.0;
		}
		self.resistance.1 = true;
	}
}