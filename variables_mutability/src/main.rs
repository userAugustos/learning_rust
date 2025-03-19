const POINTS: u32 = 100;

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
    // precisamos ver a shadowing como efetivamente criando uma nova variavel, usando do valor antigo

		let x = 5;

		let x = x + 1;

		{
			let x = x * 2;
			println!("The value of x in the inner scope is: {}", x);
		}

		println!("The value of x is: {}", x);

		// Aqui x, nao e mutavel, mas conseguimos re atribuir o valor de x usando do seu valor anterior, usando `let` sem o let, receberiamos um erro:

		//x = 4; error[E0384]: cannot assign twice to immutable variable `x`

		// ja com uma variavel mutavel

		let mut y = 5;

		y = 1;

		println!("The value of y is: {}", y);

		// Entao por que shadowing contra mut? Bom so pensar na definiçao padrao:
		// Com shadowing estamos re criando a variavel, podemos mudar tipo da variavel ou trabalhar com a referencia em tempo de execucao
		// Com mut, estamos mudando o valor da variavel, mas nao podemos mudar o tipo da variavel

		// Com shadowing

		// Vamos de uma string para um int, sem problemas
		let spaces = "   ";
    let spaces = spaces.len();

		println!("The value of spaces is: {}", spaces);

		// Com mut

		let mut mut_spaces = "   ";
		// mut_spaces = mut_spaces.len(); error[E0308]: mismatched types
		// mas ainda podemos:

		mut_spaces = " "; // mudar o valor da variavel de forma mais simples, consumindo menos memoria.
}

fn concatenate_string() {
    let mut name: String = "Felipe".to_owned(); // obviously need to be mutable
    name.push_str(" Augustos"); // Adding last name

    println!("{name}"); // => Felipe Augustos

    let address = "Rua X".to_owned();

    let full_address = address.clone() + "Bairro XY";

    println!("{full_address}");
}
