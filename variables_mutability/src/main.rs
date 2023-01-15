const POINTS: u32 = 100;

const soldier = {
    name: 'adsf'; 
}

fn main() {
    calc_with_mult();
    calc_constants();
    shadowing_vars();
    concatenate_string();
}

fn calc_constants() {
    const MULTIPLICATOR: u32 = 5;
    let math: u32 = MULTIPLICATOR * POINTS; // so here we are mutiplaying a global const and one const from this fn, they could be in a const as well, but i want it in a let

    println!("Multiplying {POINTS} * {MULTIPLICATOR}: {}", math)
}

fn calc_with_mult() {
    let mut divisor = 5;
    divisor = divisor + 1; // multable variable

    let math: u32 = divisor * POINTS;

    println!("Dividing {divisor} / {POINTS}: {math}")
}

fn shadowing_vars() {
    let shadow_var = "clean var";

    let shadow_var = "shadowing var";

    println!("{shadow_var}");
}

fn concatenate_string() {
    let mut name: String = "Felipe".to_owned(); // obviously need to be mutable
    name.push_str(" Augustos"); // Adding last name

    println!("{name}"); // => Felipe Augustos

    let address = "Rua X".to_owned();

    let full_address = address.clone() + "Bairro XY";

    println!("{full_address}");
}
