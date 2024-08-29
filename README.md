## Rust

#### Variáveis e Mutabilidade

Em Rust qualquer variável definida é Imutável, ou seja:

```rust
let x = 5;
```

Está variável agora é Imutável, ou seja, não pode ser reescrita, 'x' não pode ter seu valor sobrescrito, você terá um erro de compilação, já que o compilador rust, já aponta os problemas no seu código.

**Mas mutabilidade é essencial e util para o desenvolvimento de softwares**, então como rust resolve a necessidade ou a possibilidade de alteração de valores?

```rust
let mut x = 5;
```

Adicionando `mut`na declaração de uma variável, permite que este valor mude, além de indicar na leitura da variável que há alteração ou possibilidade de alteração da variável.

```rust
fn main() {
    let mut x = 5;
    println!("X is {}", x);
    x = 6;
    println!("Now X is {}", x);
}
```

### Variáveis e Consts

Em Rust as constantes tem uma definição e trabalho ainda mais rigido, do que você (como eu) pode trazer de conhecimento a partir de linguagens como Javascript ou php.

1. Rust você não pode usar `mut`na definição da constante:
   - Afinal, constantes não podem ser mutáveis.

```rust
const mut points = 4; // ERROR DE COMPILAÇÃO
```

2. O Tipo de valor da constante **precisa** ser definido na declaração dela.
   - Data Type é imperativo no Rust, importante as hell.

```rust
const points = 100; // ERROR DE COMPILAÇÃO
---------------
const points: u32 = 100; //SUCCESS
```

3. Constantes podem ser declaradas em qualquer escopo, e geralmente estarão no maior escopo da função ou escopo global (fora da `fn main()` ).
4. Constantes em Rust não podem ser definidas como resultado de algo que possa mudar ou valor que só possa ser calculado em tempo de execução, ou seja, uma `const` não poderia ser o resultado de uma função que executa uma chamada externa que o resultado depende em tempo de execução.
   - Vemos muito isso em javascript ou typescript ou até python, `const = func`, no geral isso não vai acontecer em rust.

- Importante ressaltar que você não vai conseguir simplesmente passar um `non-constant value` para uma const, entenda:

  ```rust
  const POINTS: u32 = 100;

  fn main() {
      let mut number = 5;
      const MATH: u32 = number * POINTS;

      println!("points * x: {}", MATH)
  } // ERROR COMPILING: error[E0435]: attempt to use a non-constant value in a constant
  ```

  Nesse caso, não estamos nem passando o valor direto, mas um resultado de um multiplicação de um valor constante com um valor mutável, e isso da erro de compilação.

  ### Shadowing

  No rust assim como em quase todas as linguagens de programação, temos o shadowing de variaveis, onde atribuímos novos valores a variaveis já definidas com valores anteriores, claro, isso não se aplica a constantes.

  O Conceito de shadowing deve ser bem intuitivo e claro, já que, naturalmente, com variáveis mutáveis, acabamos nos deparando com situações onde re-atribuímos valor a elas mesmas, adicionando ou removendo algo, ou alterando completamente o valor.
  Com o Shadowing, estamos essencialmente, criando uma nova variável, com o mesmo nome de uma anterior.

  Um exemplo util:

  ```rust
  fn shadowing_vars() {
      let shadow_var = "clean var";

      let shadow_var = "shadowing var";

      println!("{shadow_var}");
  } // => shadowing var
  ```

  Comparando:

  ```rust
  fn shadowing_vars() {
      let spaces = "  ";

      let spaces = spaces.len();
  } // => spaces = 2
  ```

  ```rust
  fn shadowing_vars() {
      let mut spaces = "  ";

      let spaces = spaces.len();
  } // => Error: mismatched types (Since the mutable variable is string (&str) and we tried to add a value usize)
  ```

**Concatenando Strings**

- Eu queria aqui fazer uma menção honrosa a primeira coisa que me pegou nesses primeiros dias de estudos, a forma como concatenamos strings no rust; Então, quando concatenamos strings, precisamos alocar espaço para o resultado final, no rust, isso precisa estar bem especificado com `to_owned()`, e então para concatenar, usamos `push_str()` ou `clone()` [veja mais sobre to_owned e clone](https://stackoverflow.com/questions/22264502/in-rust-what-is-the-difference-between-clone-and-to-owned) ;

  Ex with push_str:

  ```rust
  fn concatenate_string() {
      let mut name: String = "Felipe".to_owned(); // obviously need to be mutable
      name.push_str(" Augustos"); // Adding last name

      println!("{name}"); // => Felipe Augustos
  }
  ```

​ Ex with clone:

```rust
fn concatenate_string() {
    let address = "Rua X".to_owned(); // no need to be mutable here

    let full_address = address.clone() + "Bairro XY";

    println!("{full_address}"); // => Rua X Bairro XY
}
```
## Data Types

Rust é uma linguagem fortemente tipada, ou seja, você precisa declarar o tipo de dados que você está trabalhando, para o compilador.
O compilador consegue inferir tipos, baseado em usagem direta, por exemplo:

```rust
fn main() {
    let x = 5;
    let y = 10;
    let z = x + y;

    println!("The value of z is {}", z);
}
```
Nesse exemplo a cima, o compilador sabe que `x` é um `u32` e `y` é um `u32`, então ele sabe que o resultado de `x + y` também é um `u32`, e por isso, ele pode inferir a tipagem.

### Tipos de dados

#### Temos por principal, esses tipos de dados:

**Tipos Escalares**
Rust possui 4 tipos escalares, que são:

- Tipos Inteiros:
  - `u8`, `u16`, `u32`, `u64`, `u128`
  Tipos inteiros no rust, são bastante proximos a forma que usamos numeros no dia a dia, dependendo do tamanho que seu numero pode chdegar, você escolhe um tipo de acordo com o tamanho:
  ```rust
  let x: u8 = 5; // x é um u8
  let y: u16 = 5; // y é um u16
  let z: u32 = 5; // z é um u32
  ```
  1. u8: 8 bits (1 byte)
    - Intervalo: 0 a 255
    - x ocupa 1 byte na memória


  2. u16: 16 bits (2 bytes)

    - Intervalo: 0 a 65.535
    - y ocupa 2 bytes na memória

  3. u32: 32 bits (4 bytes)

    - Intervalo: 0 a 4.294.967.295
    - z ocupa 4 bytes na memória

  Outros tipos existem como vimos e variam com o seu tamanho.
  Seguiindo a forma como usamos numeros no dia a dia, rust possui uma definição inteiros positivos e negativos, sendo os positivos iniciados com `u` e negativos com `i`

  ```rust
  let x: i8 = 5; // x é um i8
  let y: i16 = 5; // y é um i16
  let z: i32 = 5; // z é um i32
  ```
  Isso apenas muda que esses numeros podem ter valores negativos.

  Então como você pode saber qual tipo de inteiro usar? Se sentir-se inseguro, as escolhas padrões do Rust geralmente são boas, e por padrão os inteiros são do tipo i32: Esse tipo geralmente é o mais rápido, até em sistemas de 64-bit. A principal situação em que você usuaria isize ou usize é indexar algum tipo de coleção.

- Tipos de Ponto Float:
