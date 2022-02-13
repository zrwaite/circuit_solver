mod commands;
#[path = "circuit.rs"] mod circuit;
#[path = "circuit/element_values.rs"] mod element_values;

#[path = "terminal.rs"] mod terminal;
#[path = "terminal/get_input.rs"] mod get_input;
#[path = "terminal/terminal_functions.rs"] mod terminal_functions;



fn main() {
    terminal::terminal::build_circuit();
}