#[path = "./circuit/node.rs"] mod node;
#[path = "./circuit/resistor.rs"] mod resistor;
#[path = "./circuit/voltage_source.rs"] mod voltage_source;
#[path = "./circuit/current_source.rs"] mod current_source;

pub struct Circuit {
	voltage_sources: Vec<voltage_source::VoltageSource>,
	current_sources: Vec<current_source::CurrentSource>,
	resistors: Vec<resistor::Resistor>,
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
		self.resistors.push(resistor::Resistor::new(resistance, name));
	}
	pub fn add_voltage_source(
		&mut self,
		name: String,
		voltage: (f32, bool)
	) {
		self.voltage_sources.push(voltage_source::VoltageSource::new(voltage, name));
	}
	pub fn add_current_source(
		&mut self,
		name: String,
		current: (f32, bool)
	) {
		self.current_sources.push(current_source::CurrentSource::new(current, name));
	}
	pub fn add_node(&mut self) {
		self.nodes.push(node::Node::new());
	}
	pub fn get_resistors(&self) -> &Vec<resistor::Resistor> {
		&self.resistors
	}
	pub fn get_nodes(&self) -> &Vec<node::Node> {
		&self.nodes
	}
	pub fn print_elements(&self) {
		for current_source in self.current_sources.iter() {
			current_source.get_values().print();
		}
		for voltage_source in self.voltage_sources.iter() {
			voltage_source.get_values().print();
		}
		for resistor in self.resistors.iter() {
			resistor.get_values().print();
		}
		
	}
}