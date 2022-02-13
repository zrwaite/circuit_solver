pub mod circuit {
	mod circuit_node;
	use circuit_node::circuit_node as node;
	mod resistor;
	use resistor::resistor as res;
	mod voltage_source;
	use voltage_source::voltage_source as vs;
	mod current_source;
	use current_source::current_source as cs;

	use crate::terminal_functions::terminal_functions as tf;
	
	
	pub struct Circuit {
		voltage_sources: Vec<vs::VoltageSource>,
		current_sources: Vec<cs::CurrentSource>,
		resistors: Vec<res::Resistor>,
		nodes: Vec<node::Node>
	}
	impl Circuit {
		pub fn new() -> Circuit{
			let new_circuit = Circuit{
				current_sources: Vec::new(),
				voltage_sources: Vec::new(),
				resistors: Vec::new(),
				nodes: Vec::new()
			};
			return new_circuit;
		}
		pub fn add_resistor(
			&mut self,
			name: String,
			resistance: (f32, bool)
		) {
			self.resistors.push(res::Resistor::new(resistance, name));
		}
		pub fn add_voltage_source(
			&mut self,
			name: String,
			voltage: (f32, bool)
		) {
			self.voltage_sources.push(vs::VoltageSource::new(voltage, name));
		}
		pub fn add_current_source(
			&mut self,
			name: String,
			current: (f32, bool)
		) {
			self.current_sources.push(cs::CurrentSource::new(current, name));
		}
		pub fn add_node(&mut self) {
			self.nodes.push(node::Node::new());
		}
		pub fn get_resistors(&self) -> &Vec<res::Resistor> {
			&self.resistors
		}
		pub fn get_nodes(&self) -> &Vec<node::Node> {
			&self.nodes
		}
		pub fn print_elements(&self) {
			for current_source in self.current_sources.iter() {
				let name_prefix = format!(" {}: ", current_source.get_name());
				tf::print_prefix(&name_prefix);
				current_source.get_values().print();
			}
			for voltage_source in self.voltage_sources.iter() {
				let name_prefix = format!(" {}: ", voltage_source.get_name());
				tf::print_prefix(&name_prefix);
				voltage_source.get_values().print();
			}
			for resistor in self.resistors.iter() {
				let name_prefix = format!(" {}: ", resistor.get_name());
				tf::print_prefix(&name_prefix);
				resistor.get_values().print();
			}
			
		}
	}
}
