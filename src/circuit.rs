mod element;
mod node;
pub fn create_circuit() -> Circuit{
	let new_circuit = Circuit{
        elements: Vec::new(),
        nodes: Vec::new()
    };
	return new_circuit;
}
pub struct Circuit {
	elements: Vec<element::Element>,
	nodes: Vec<node::Node>
}
impl Circuit {
	pub fn add_element(&mut self) {
		self.elements.push(element::create_element());
	}
	pub fn add_node(&mut self) {
		self.nodes.push(node::create_node());
	}
	pub fn get_elements(&self) -> &Vec<element::Element> {
		return &self.elements;
	}
	pub fn get_nodes(&self) -> &Vec<node::Node> {
		return &self.nodes;
	}
}