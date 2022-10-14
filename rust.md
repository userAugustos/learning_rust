## Rust





#### Variáveis e Multabilidade

Em Rust qualquer variável definida é imutavel, ou seja: 
```rust
let x = 5;
```

Está variável agora é imultavel, ou seja, não pode ser reescrita, 'x' não pode ter seu valor sobrescrito, você terá um erro de compilação, já que o compilador rust, já aponta os problemas no seu código.

**Mas mutabilidade é essencial e util para o desenvolvimento de softwares**, então como rust resolve a necessidade ou a possibilidade de alteração de valores?

```rust
let mut x = 5;
```

Adicionando ``mut``na declarção de uma variável, permite que este valor mude, além de indicar na leitura da variável que há alteração ou possibilidade de alteração da variável.

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

1. Rust você não pode usar ``mut``na definição da constante: 
   - Afinal, constantes não podem ser mutaveis.

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

3. Constantes podem ser declaradas em qualquer escopo, e geralmente estarão no maior escopo da função ou escopo global (fora da ``fn main()`` ).
4. Constantes em Rust não podem ser definidas como resultado de algo que possa mudar ou valor que só possa ser calculado em tempo de execução, ou seja, uma ``const`` não poderia ser o resultado de uma função que executa uma chamada externa que o resultado depende em tempo de execução.
   -  Vemos muito isso em javascript ou typescript ou até python, ``const = func``, no geral isso não vai acontecer em rust.

- Importante ressaltar que você não vai conseguir simplesmente passar um ``non-constant value`` para uma const, entenda: 

  ```rust
  const POINTS: u32 = 100;
  
  fn main() {
      let mut number = 5;
      const MATH: u32 = number * POINTS;
  
      println!("points * x: {}", MATH)
  } // ERROR COMPILING: error[E0435]: attempt to use a non-constant value in a constant
  ```

  Nesse caso, não estamos nem passando o valor direto, mas um resultado de um multiplicação de um valor constante com um valor multável, e isso da erro de compilação.

  

  
