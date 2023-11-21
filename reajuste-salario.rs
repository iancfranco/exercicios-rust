use std::io;

fn main() {
    println!("Digite o valor do seu salário:");
    let mut salario = String::new();
    io::stdin().read_line(&mut salario).expect("Input incorreto.");
    let salario: f64 = salario.trim().parse().expect("Insira um valor numérico.");
    println!("Qual o valor do reajuste em %?");
    let mut reajuste = String::new();
    io::stdin().read_line(&mut reajuste).expect("Input incorreto.");
    let reajuste: f64 = reajuste.trim().parse().expect("Insira um valor numérico.");
    let valor_reajuste = (salario / 100.0) * reajuste;
    println!("O valor do reajuste é de R$ {:.2} e o novo salário é de R$ {:.2}", valor_reajuste, salario + valor_reajuste);
}
