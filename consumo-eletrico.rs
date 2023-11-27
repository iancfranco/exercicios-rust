use std::io;

fn main() {
  while 1!=0{
    let mut preco_kwh = 0;
    println!("1 - Barreiro");
    println!("2 - Centro-Sul");
    println!("3 - Leste");
    println!("4 - Nordeste");
    println!("5 - Noroeste");
    println!("6 - Norte");
    println!("7 - Oeste");
    println!("8 - Pampulha");
    println!("9 - Venda Nova");
    println!("Digite sua região:");
    let mut regiao = String::new();
    io::stdin().read_line(&mut regiao).expect("Input incorreto.");
    let regiao: f64 = regiao.trim().parse().expect("Insira um valor numérico");
  
    if regiao == 1.0{
      println!("Região Barreiro - Busca de bairro pela inicial
      1 - A
      2 - B
      3 - C
      4 - D
      5 - E
      6 - F
      7 - I
      8 - J
      9 - L
      10 - M
      11 - N
      12 - O
      13 - P
      14 - S
      15 - T
      16 - V
      Por qual inicial deseja buscar?");
      let mut inicial = String::new();
      io::stdin().read_line(&mut inicial).expect("Input incorreto.");
      let inicial: f64 = inicial.trim().parse().expect("Insira um valor numérico");
      if inicial == 1.0{
        println!("Bairros na região Barreiro com inicial A
        1 - Aldemar Maldonado
        2 - Águas Claras
        3 - Alta Tensão
        4 - Alta Tensão I
        5 - Alto das Antenas
        6 - Araguaia
        7 - Átila de Paiva
        Selecione seu bairro:");
        let mut bairro = String::new();
        io::stdin().read_line(&mut bairro).expect("Input incorreto.");
        let bairro: f64 = bairro.trim().parse().expect("Insira um valor numérico");
        if bairro == 1.0{
          println!("Bairro selecionado: Aldemar Maldonado");
          println!("Valor do KWh: R$0,3841");
          let preco_kwh = 0.38;
        }
        else if bairro == 2.0{
          println!("Bairro selecionado: Águas Claras");
          println!("Valor do KWh: R$0,4102");
          let preco_kwh = 0.41;
        }
        else if bairro == 3.0{
          println!("Bairro selecionado: Alta Tensão");
          println!("Valor do KWh: R$0,3212");
          let preco_kwh = 0.32;
        }
        else if bairro == 4.0{
          println!("Bairro selecionado: Alta Tensão I");
          println!("Valor do KWh: R$0,3592");
          let preco_kwh = 0.35;
        }
        else if bairro == 5.0{
          println!("Bairro selecionado: Alto das Antenas");
          println!("Valor do KWh: R$0,3024");
          let preco_kwh = 0.30;
        }
        else if bairro == 6.0{
          println!("Bairro selecionado: Araguaia");
          println!("Valor do KWh: R$0,2994");
          let preco_kwh = 0.34;
        }
        else if bairro == 7.0{
          println!("Bairro selecionado: Átila de Paiva");
          println!("Valor do KWh: R$0,3795");
          let preco_kwh = 0.37;
        }
        else{
          println!("Bairro não encontrado.");
        }
      }
        
      else if inicial == 2.0{
        println!("Bairros na região Barreiro com inicial B
        1 - Bairro das Indústrias I
        2 - Bairro Novo das Indústrias
        3 - Barreiro
        4 - Bernadete
        5 - Bonsucesso
        6 - Brasil Industrial
        Selecione seu bairro:");
        let mut bairro = String::new();
        io::stdin().read_line(&mut bairro).expect("Input incorreto.");
        let bairro: f64 = bairro.trim().parse().expect("Insira um valor numérico");
        if bairro == 1.0{
          println!("Bairro selecionado: Bairro das Indústrias I");
          println!("Valor do KWh: R$0,4138");
          let preco_kwh = 0.34;
        }
        else if bairro == 2.0{
          println!("Bairro selecionado: Bairro Novo das Indústrias");
          println!("Valor do KWh: R$0,4138");
          let preco_kwh = 0.29;
        }
        else if bairro == 3.0{
          println!("Bairro selecionado: Barreiro");
          println!("Valor do KWh: R$0,4138");
          let preco_kwh = 0.38;
        }
        else if bairro == 4.0{
          println!("Bairro selecionado: Bernadete");
          println!("Valor do KWh: R$0,4138");
          let preco_kwh = 0.41;
        }
        else if bairro == 5.0{
          println!("Bairro selecionado: Bonsucesso");
          println!("Valor do KWh: R$0,4138");
          let preco_kwh = 0.46;
        }
        else if bairro == 6.0{
          println!("Bairro selecionado: Brasil Industrial");
          println!("Valor do KWh: R$0,4138");
          let preco_kwh = 0.35;
        }
      }
  
      else if inicial == 2.0{
          println!("Bairros na região Barreiro com inicial C
          1 - Cardoso
          2 - Castanheira
          3 - CDI Jatobá
          4 - Conjunto Bonsucesso
          5 - Conjunto Jatobá
          6 - Corumbiara
          Selecione seu bairro:");
          let mut bairro = String::new();
          io::stdin().read_line(&mut bairro).expect("Input incorreto.");
          let bairro: f64 = bairro.trim().parse().expect("Insira um valor numérico");
          if bairro == 1.0{
            println!("Bairro selecionado: Cardoso");
            println!("Valor do KWh: R$0,4138");
            let preco_kwh = 0.34;
          }
          else if bairro == 2.0{
            println!("Bairro selecionado: Castanheira");
            println!("Valor do KWh: R$0,4138");
            let preco_kwh = 0.29;
          }
          else if bairro == 3.0{
            println!("Bairro selecionado: CDI Jatobá");
            println!("Valor do KWh: R$0,4138");
            let preco_kwh = 0.38;
          }
          else if bairro == 4.0{
            println!("Bairro selecionado: Conjunto Bonsucesso");
            println!("Valor do KWh: R$0,4138");
            let preco_kwh = 0.41;
          }
          else if bairro == 5.0{
            println!("Bairro selecionado: Conjunto Jatobá");
            println!("Valor do KWh: R$0,4138");
            let preco_kwh = 0.46;
          }
          else if bairro == 6.0{
            println!("Bairro selecionado: Corumbiara");
            println!("Valor do KWh: R$0,4138");
            let preco_kwh = 0.35;
          }
      }
  
      else if inicial == 3.0{
        println!("Bairros na região Barreiro com inicial D
        1 - Diamante
        2 - Distrito Industrial do Jatobá
        Selecione seu bairro:");
        let mut bairro = String::new();
        io::stdin().read_line(&mut bairro).expect("Input incorreto.");
        let bairro: f64 = bairro.trim().parse().expect("Insira um valor numérico");
        if bairro == 1.0{
          println!("Bairro selecionado: Diamante");
          println!("Valor do KWh: R$0,4138");
          let preco_kwh = 0.34;
        }
        else if bairro == 2.0{
          println!("Bairro selecionado: Distrito Industrial do Jatobá");
          println!("Valor do KWh: R$0,4138");
          let preco_kwh = 0.29;
        }
      }
  
      else if inicial == 4.0{
        println!("Bairros na região Barreiro com inicial E
        1 - Ernesto do Nascimento
        2 - Esperança
        Selecione seu bairro:");
        let mut bairro = String::new();
        io::stdin().read_line(&mut bairro).expect("Input incorreto.");
        let bairro: f64 = bairro.trim().parse().expect("Insira um valor numérico");
        if bairro == 1.0{
          println!("Bairro selecionado: Ernesto do Nascimento");
          println!("Valor do KWh: R$0,4138");
          let preco_kwh = 0.34;
        }
        else if bairro == 2.0{
          println!("Bairro selecionado: Esperança");
          println!("Valor do KWh: R$0,4138");
          let preco_kwh = 0.29;
        }
      }
  
      else if inicial == 5.0{
        println!("Bairros na região Barreiro com inicial F
        1 - Flávio de Oliveira
        2 - Flávio Marques Lisboa
        Selecione seu bairro:");
        let mut bairro = String::new();
        io::stdin().read_line(&mut bairro).expect("Input incorreto.");
        let bairro: f64 = bairro.trim().parse().expect("Insira um valor numérico");
        if bairro == 1.0{
          println!("Bairro selecionado: Flávio de Oliveira");
          println!("Valor do KWh: R$0,4138");
          let preco_kwh = 0.34;
        }
        else if bairro == 2.0{
          println!("Bairro selecionado: Flávio Marques Lisboa");
          println!("Valor do KWh: R$0,4138");
          let preco_kwh = 0.29;
        }
      }
    }
    println!("Insira o consumo de energia em KWh:");
    let mut consumo = String::new();
    io::stdin().read_line(&mut consumo).expect("Input incorreto.");
    let consumo: i32 = consumo.trim().parse().expect("Insira um valor numérico");
    let valor_total = consumo * preco_kwh;
    println!("Valor total a ser pago: R$ {:.2}", valor_total);
  }
}
