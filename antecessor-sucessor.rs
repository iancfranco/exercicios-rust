use std::io;

fn main() {
    println!("Digite seu primeiro valor:");
    let mut v1 = String::new();
    io::stdin().read_line(&mut v1).expect("Input incorreto.");
    let v1: i32 = v1.trim().parse().expect("Insira um valor numérico.");
    let sucessor = v1 + 1;
    let antecessor = v1 - 1;
    println!("O antecessor do número {} é {}. Seu sucessor é {}.", v1, antecessor, sucessor);
}
