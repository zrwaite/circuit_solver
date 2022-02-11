#[path = "circuit/resistor.rs"] mod resistor;
mod terminal;
mod circuit;


fn main() {
    terminal::build_circuit();
	// let colours = terminal::read_colours();
    // let resistance = resistor::get_resistance_from_colours(colours);
    // println!("Resistance: {}Ω", resistance);
}