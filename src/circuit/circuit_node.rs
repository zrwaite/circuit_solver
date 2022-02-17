pub mod circuit_node {
	pub struct Node {
		pub node_voltage: f32,
		pub resistors: Vec<usize>, //Index
		pub current_sources: Vec<(usize, bool)>, //Index, positive direction
		pub voltage_sources: Vec<(usize, bool)> //Index, positive terminal
	}
	
	impl Node {
		pub fn new() -> Node{
			let new_node= Node{
				node_voltage: 0.0,
				resistors: Vec::new(),
				voltage_sources: Vec::new(),
				current_sources: Vec::new()
			};
			return new_node;
		}
		pub fn add_resistor(&mut self, index: usize) {
			self.resistors.push(index);
		}
		pub fn add_current_source(&mut self, index:usize, positive_direction: bool) {
			self.current_sources.push((index, positive_direction));
		}
		pub fn add_voltage_source(&mut self, index:usize, positive_terminal: bool) {
			self.voltage_sources.push((index, positive_terminal));
		}
	}
}
