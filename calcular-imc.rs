use std::io;

fn main() {
    println!("Digite o seu peso:");
    let mut peso = String::new();
    io::stdin().read_line(&mut peso).expect("Input incorreto.");
    let peso: f64 = peso.trim().parse().expect("Insira um valor numérico");

    println!("Digite sua altura:");
    let mut altura = String::new();
    io::stdin().read_line(&mut altura).expect("Input incorreto.");
    let altura: f64 = altura.trim().parse().expect("Insira um valor numérico");

    let imc = peso / (altura * altura);
    println!("Seu IMC é: {:.2}", imc);
    if imc < 18.5 {
      println!("Você está abaixo do peso.");
    } else if imc < 25.0 {
      println!("Você está com o peso normal.");
    } else if imc < 30.0 {
      println!("Você está com sobrepeso.");
    } else if imc < 35.0 {
      println!("Você está com obesidade grau 1.");
    } else if imc < 40.0 {
      println!("Você está com obesidade grau 2.");
    } else {
      println!("Você está com obesidade grau 3.");
    }
}
