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

  Um exemplo util:

  ```rust
  fn shadowing_vars() {
      let shadow_var = "clean var";

      let shadow_var = "shadowing var";

      println!("{shadow_var}");
  } // => shadowing var
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

    println!("{full_address}"); // => Felipe Augustos
}
```
