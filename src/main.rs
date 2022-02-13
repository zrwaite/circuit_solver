mod commands;
#[path = "circuit.rs"] mod circuit;
#[path = "circuit/element_values.rs"] mod element_values;

#[path = "terminal.rs"] mod terminal;
#[path = "terminal/get_input.rs"] mod get_input;
#[path = "terminal/terminal_functions.rs"] mod terminal_functions;



fn main() {
    terminal::build_circuit();
	// let colours = terminal::read_colours();
    // let resistance = resistor::get_resistance_from_colours(colours);
    // println!("Resistance: {}Ω", resistance);
}