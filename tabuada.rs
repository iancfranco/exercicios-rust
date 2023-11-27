use std::io;
fn main() {
  println!("Valor a multiplicar: ");
  let mut valor = String::new();
  io::stdin().read_line(&mut valor)
      .expect("Input incorreto.");
  let valor: i32 = valor.trim()
      .parse()
      .expect("O valor não é numérico.");
  println!("Quantidade de vezes a multiplicar: ");
  let mut temp_v = 0;
  let mut tab = String::new();
  io::stdin().read_line(&mut tab)
      .expect("Input incorreto.");
  let mut tab: i32 = tab.trim()
      .parse()
      .expect("O valor não é numérico.");

  while temp_v < tab{
    println!("{}x{} = {}", valor, temp_v, valor * temp_v);
    temp_v += 1;
  };
}
