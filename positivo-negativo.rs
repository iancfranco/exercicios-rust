use std::io;

fn main() {
    println!("Digite seu primeiro valor:");
    let mut v1 = String::new();
    io::stdin().read_line(&mut v1).expect("Failed to read input");
    let v1: i32 = v1.trim().parse().expect("Please type a number!");
    if v1 > 0 {
        println!("O valor {} é positivo.", v1);
    } else if v1 == 0 {
        println!("O valor {} é zero.", v1);
    } else {
      println!("O valor é negativo.")
    }
}
