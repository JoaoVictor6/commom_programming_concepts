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


Pra isso funcionar, basta **declarar** essa variável como **mutável**:**
```rust
fn variables_example(){
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```
