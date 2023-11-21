use std::io;

fn main() {
    println!("Digite o nome do aluno:");
    let mut nome = String::new();
    io::stdin().read_line(&mut nome).expect("Input inválido.");

    println!("Digite a primeira nota:");
    let mut nota1 = String::new();
    io::stdin().read_line(&mut nota1).expect("Input inválido.");
    let nota1: f64 = nota1.trim().parse().expect("Insira um valor numérico.");

    println!("Digite a segunda nota:");
    let mut nota2 = String::new();
    io::stdin().read_line(&mut nota2).expect("Input inválido.");
    let nota2: f64 = nota2.trim().parse().expect("Insira um valor numérico.");

    println!("Digite a terceira nota:");
    let mut nota3 = String::new();
    io::stdin().read_line(&mut nota3).expect("Input inválido.");
    let nota3: f64 = nota3.trim().parse().expect("Insira um valor numérico.");

    println!("Digite a quarta nota:");
    let mut nota4 = String::new();
    io::stdin().read_line(&mut nota4).expect("Entrada inválida.");
    let nota4: f64 = nota4.trim().parse().expect("Insira um valor numérico.");

    let media = (nota1 + nota2 + nota3 + nota4) / 4.0;
    println!("A média das notas é: {:.2}", media);
    if media >= 7.0 {
        println!("O aluno {} foi aprovado.", nome.trim());
    } else {
        println!("O aluno {} foi reprovado.", nome.trim());
    }
}
