mod circuit;

fn main() {
    let mut new_circuit = circuit::create_circuit();
    new_circuit.add_element(
        (24.0, true),
        (24.0, true),
        (1.0, true),
        (24.0, true)
    );
    let element = new_circuit.get_elements();
    element[0].print();
}
