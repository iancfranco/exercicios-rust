use std::io;

fn main() {
    println!("Digite seu primeiro valor:");
    let mut v1 = String::new();
    io::stdin().read_line(&mut v1).expect("Input incorreto.");
    let v1: i32 = v1.trim().parse().expect("Insira um valor numérico.");
    println!("Digite seu segundo valor:");
    let mut v2 = String::new();
    io::stdin().read_line(&mut v2).expect("Input incorreto.");
    let v2: i32 = v2.trim().parse().expect("Insira um valor numérico.");
    if v1 == v2{
      println!("Os valores são iguais.");
      println!("A soma dos valores é {}.", v1+v2);
    } else {
      println!("Os valores não são iguais.");
      println!("A subtração dos valores é {}.", v1-v2);
    }
}
