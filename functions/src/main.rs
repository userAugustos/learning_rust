struct Car {
	model: String,
	year: u32,
	base_price: f64,
	available: bool
}

struct CalculateBoxProps {
	width: f64,
	height: f64,
	depth: f64,
}

fn main() {
  calc_box_volume(CalculateBoxProps {
		width: 10.0,
		height: 10.0,
		depth: 10.0,
	});

	// Exemplo de Expression e Statement com venda de carro


	let mut my_car = Car {
		model: String::from("Sedan XYZ"),
		year: 2019,
		base_price: 45000.0,
		available: true,
	};

	// Ignoramos o retorno default `()`, vc nunca vai usar esse retorno.
	calc_final_price(&my_car);

	// Pegamos o retorno da expression
	let final_price = calc_final_price(&my_car);
	println!("Final price: $ {:.2}", final_price);

	// Statement de novo, vamos alterar a `my_car`
	register_sale(&mut my_car);

	// De novo, expression, a variavel e um statement, a logica que determina o valor, expression
	let sale_status = if my_car.available { "available" } else { "sold" };

	println!("Car status: {}", sale_status);
}

fn calc_box_volume(CalculateBoxProps { width, height, depth }: CalculateBoxProps) {
	let volume = width * height * depth;
	println!("volume: {}", volume);
}

// Exemplo de funçao que é um statement - não retorna um valor
fn register_sale(car: &mut Car) {
	println!("Registering sale of car: {}", car.model);

	// statement - faz uma açao, referenciando o parametro que recebemos (transformado a mutavel), nao retorna nada
	car.available = false;

	println!("Car {} is now unavailable", car.model);
}

// Exemplo de expression - retorna um valor
fn calc_final_price(car: &Car) -> f64 {
	// expression como processamento retornando valor para um statement
	let year_rate = if car.year < 2020 { 0.9 } else { 1.0 };

	// Expression como block logico, esse bloco logico retorna um valor
	{
		let base_discount = 0.05;
		let price_with_discount = car.base_price * (1.0 - base_discount);

		// Essa linha tem return implicito, por que e uma expressao (retorna um valor) e nao possui `;`
		price_with_discount * year_rate
	} // esse bloco inteiro e uma expressao, que retorna um valor - esse valor e a ultima expressao do bloco. Poderia ser um return normalmente.
}