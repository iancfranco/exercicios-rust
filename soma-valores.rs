use std::io;

fn main() {
    println!("Digite seu primeiro valor:");
    let mut v1 = String::new();
    io::stdin().read_line(&mut v1).expect("Input incorreto.");
    let v1: i32 = v1.trim().parse().expect("O valor não é numérico.");

    println!("Digite seu segundo valor:");
    let mut v2 = String::new();
    io::stdin().read_line(&mut v2).expect("Input incorreto.");
    let v2: i32 = v2.trim().parse().expect("O valor não é numérico.");

    println!("Digite seu terceiro valor:");
    let mut v3 = String::new();
    io::stdin().read_line(&mut v3).expect("Input incorreto.");
    let v3: i32 = v3.trim().parse().expect("O valor não é numérico.");

    println!("A soma entre o primeiro e o segundo é:");
    println!("{}", v1 + v2);
    let resultado = v1 + v2;
    println!("O resultado menos o terceiro valor é:");
    println!("{}", resultado - v3);
}
