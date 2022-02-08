mod circuit;

fn main() {
    let mut new_circuit = circuit::create_circuit();
    new_circuit.add_element(
        (5.0, true),
        (5.0, true),
        (1.0, true)
    );
    let voltage = new_circuit.get_elements()[0].get_voltage();
    let current = new_circuit.get_elements()[0].get_current();
    if current.1 {
        println!("current: {}", current.0);
    } else {
        println!("current undefined");
    }
}
