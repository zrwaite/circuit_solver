pub fn create_element_values(
	voltage: (f32, bool),
	resistance: (f32, bool),
	current: (f32, bool),
	power: (f32, bool)
) -> ElementValues{
	//Constructor
	let mut new_element = ElementValues{
		voltage: voltage,
		resistance: resistance,
		current: current,
		power: power
    };
	new_element.validate_values();
	new_element.try_calc();
	return new_element;
}
pub struct ElementValues {
	voltage: (f32, bool),
	resistance: (f32, bool),
	current: (f32, bool),
	power: (f32, bool)
}

impl ElementValues {
	/* Getters */
	pub fn get_voltage(&self) -> (f32, bool) {self.voltage}
	pub fn get_resistance(&self) -> (f32, bool) {self.resistance}
	pub fn get_current(&self) -> (f32, bool) {self.current}
	pub fn get_power(&self) -> (f32, bool) {self.power}

	pub fn print(&self) {
		//Prints the values of the object
		let voltage = if self.voltage.1 { self.voltage.0.to_string() } else { String::from("undefined") };
		let current = if self.current.1 { self.current.0.to_string() } else { String::from("undefined") };
		let resistance = if self.resistance.1 { self.resistance.0.to_string() } else { String::from("undefined") };
		let power = if self.power.1 { self.power.0.to_string() } else { String::from("undefined") };
		println!("<Voltage: {}V> <Resistance: {}Ω> <Current: {}A> <Power: {}W>", voltage, resistance, current, power);
	}
	
	fn validate_values(&self) {
		let mut num_vars = 0; //Number of standard variables calculated
		if self.voltage.1 {num_vars+=1}
		if self.current.1 {num_vars+=1}
		if self.resistance.1 {num_vars+=1}

		//Confirm that the values match if there are additional values
		if num_vars == 3{
			//Validate V = I * R
			let error = self.voltage.0 - self.current.0 * self.resistance.0;
			if error.abs() > 0.1 {panic!("Invalid value added to element");}
		} else if num_vars>1 && self.power.1 {
			if self.current.1 && self.voltage.1 {
				// Validate P = I * V
				let error = self.power.0 - self.current.0 * self.voltage.0;
				if error.abs() > 0.1 {panic!("Invalid value added to element");}
			} else if self.current.1 && self.resistance.1 {
				// Validate P = I^2 / V
				let error = self.power.0 - self.current.0.powf(2.0) * self.resistance.0;
				if error.abs() > 0.1 {panic!("Invalid value added to element");}
			} else if self.voltage.1 && self.resistance.1 {
				// Validate P = V^2 / R
				let error = self.power.0 - self.voltage.0.powf(2.0) / self.resistance.0;
				if error.abs() > 0.1 {panic!("Invalid value added to element");}
			}
		}
	}
	fn try_calc(&mut self) {
		//Using the values of the attributes, find additional values that can be calculated. 
		let mut num_vars = 0; //Number of standard variables calculated
		if self.voltage.1 {num_vars+=1}
		if self.current.1 {num_vars+=1}
		if self.resistance.1 {num_vars+=1}
		//Check for ohm's law
		if num_vars == 2 {self.ohm_law()}
		//Check for power calculations
		if num_vars>1 && !self.power.1 {
			self.calc_power();
		} else if num_vars==1 && self.power.1 {
			self.unpack_power();
		}
	}
	fn calc_power(&mut self) {
		assert!(!self.power.1);
		assert!(self.current.1);
		assert!(self.resistance.1);
		assert!(self.voltage.1);
		self.power.0 = self.current.0 * self.voltage.0;
		self.power.1 = true;
	}
	fn unpack_power(&mut self) {
		assert!(self.power.1);
		if self.current.1 {
			//Calculate voltage, V = P / I
			assert!(!self.voltage.1);
			if self.current.0 == 0.0 {
				self.voltage.0 = f32::INFINITY;
			} else {
				self.voltage.0 = self.power.0 / self.current.0;
			}
			self.voltage.1 = true;
			self.ohm_law();
		} else if self.voltage.1 {
			//Calculate current, I = P / V
			assert!(!self.current.1);
			if self.voltage.0 == 0.0 {
				self.current.0 = f32::INFINITY;
			} else {
				self.current.0 = self.power.0 / self.voltage.0;
			}
			self.current.1 = true;
			self.ohm_law();
		} else if self.resistance.1 {
			//Calculate voltage, V = √(P * R)
			assert!(!self.voltage.1);
			self.voltage.0 = (self.power.0 * self.resistance.0).sqrt();
			self.voltage.1 = true;
			self.ohm_law();
		}
	}
	fn ohm_law(&mut self) {
		if self.current.1 && self.resistance.1 {
			//Calculate Voltage: V = I * R
			assert!(!self.voltage.1);
			self.voltage.0 = self.current.0 * self.resistance.0;
			self.voltage.1 = true;
		} else if self.voltage.1 && self.current.1 {
			//Calculate resistance: R = V / I
			assert!(!self.resistance.1);
			if self.current.0 == 0.0 {self.resistance.0 = f32::INFINITY} 
			else {self.resistance.0 = self.voltage.0 / self.current.0;}
			self.resistance.1 = true;
		} else if self.resistance.1 && self.voltage.1 {
			//Calculate current: I = V / R
			assert!(!self.current.1);
			if self.resistance.0 == 0.0 {self.current.0 = f32::INFINITY;} 
			else {self.current.0 = self.voltage.0 / self.resistance.0;}
			self.current.1 = true;
		}
	}
}