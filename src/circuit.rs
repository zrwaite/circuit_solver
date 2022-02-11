#[path = "./circuit/node.rs"] mod node;
#[path = "./circuit/resistor.rs"] mod resistor;

pub struct Circuit {
	resistors: Vec<resistor::Resistor>,
	nodes: Vec<node::Node>
}
impl Circuit {
	pub fn new() -> Circuit{
		let new_circuit = Circuit{
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
	pub fn add_node(&mut self) {
		self.nodes.push(node::Node::new());
	}
	pub fn get_resistors(&self) -> &Vec<resistor::Resistor> {
		&self.resistors
	}
	pub fn get_nodes(&self) -> &Vec<node::Node> {
		&self.nodes
	}
}