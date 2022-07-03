## Variables

Em rust, variáveis são naturalmente imutáveis. podemos mudar isso.
```rust
fn variables_example(){
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```
| Saporra nem roda tá. queria que o javascript fosse assim kkkkk


Pra isso funcionar, basta **declarar** essa variável como **mutável**:
```rust
fn variables_example(){
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

## Constants

São bem parecidas com a let, porém ela realmente não podem ser mutáveis. 
A gente cria uma variavel constante usando a palavra __const__.
Essas belezinhas podem ser declaradas até em escope global. Além disso,
uma constante tem que ter um valor **constante**, não pode ser o retorno de alguma 
função ou algo do tipo, por exemplo:
```rust
// ERRADO
const EXAMPLE = fn();
// CERTO
const EXAMPLE = 123;
```

| Dica: O nome das constantes é sempre em CAPS_LOCK. 
