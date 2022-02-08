mod circuit;

fn main() {
    let mut new_circuit = circuit::create_circuit();
    new_circuit.add_element();
    println!("test print: {}", new_circuit.get_elements()[0].get_current());
}
