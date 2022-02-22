mod parse;
#[path = "circuit.rs"] mod circuit;
#[path = "circuit/element_values.rs"] mod element_values;
#[path = "circuit/circuit_node.rs"] mod circuit_node;
#[path = "circuit/resistor.rs"] mod resistor;

#[path = "terminal.rs"] mod terminal;
#[path = "terminal/get_input.rs"] mod get_input;
#[path = "terminal/terminal_functions.rs"] mod terminal_functions;



fn main() {
    terminal::terminal::build_circuit();
}