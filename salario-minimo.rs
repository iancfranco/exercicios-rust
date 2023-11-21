use std::io;

fn main() {
    println!("Digite o valor do seu salário:");
    let mut salario = String::new();
    io::stdin().read_line(&mut salario).expect("Input incorreto.");
    let salario: f64 = salario.trim().parse().expect("Insira um valor numérico.");
    println!("Qual o valor do salário mínimo?");
    let mut salario_minimo = String::new();
    io::stdin().read_line(&mut salario_minimo).expect("Input incorreto.");
    let salario_minimo: f64 = salario_minimo.trim().parse().expect("Insira um valor numérico.");
    let qtd_salarios_minimo = salario / salario_minimo;
    println!("Você recebe {} salários mínimos.", qtd_salarios_minimo);
}
