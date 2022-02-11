
pub struct Node {
	node_voltage: f32
}

impl Node {
	pub fn new() -> Node{
		let new_node= Node{
			node_voltage: 0.0
		};
		return new_node;
	}
}