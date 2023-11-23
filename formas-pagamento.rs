use std::io;

fn main() {
    println!("Valor:");
    let mut valor = String::new();
    io::stdin().read_line(&mut valor)
        .expect("Input incorreto.");
    let valor: f64 = valor.trim()
        .parse()
        .expect("O valor não é numérico.");

    println!("Formas de pagamento:");
    println!("1 - Dinheiro");
    println!("2 - Pix");
    println!("3 - Débito");
    println!("4 - Crédito");
    println!("5 - Vale");
    let mut forma_pgto = String::new();
    io::stdin().read_line(&mut forma_pgto)
        .expect("Input incorreto.");
    let forma_pgto: i32 = forma_pgto.trim()
        .parse()
        .expect("O valor não é numérico.");

    match forma_pgto {
        1 => {
            println!("Você pagará R${} utilizando Dinheiro.", valor);
            let final_value = valor * 0.85;
            println!("Você receberá 15% de desconto.");
            println!("O valor final será de R${}.", final_value);
        }
        2 => {
            println!("Você pagará R${} utilizando Pix.", valor);
            let final_value = valor * 0.90;
            println!("Você receberá 10% de desconto.");
            println!("O valor final será de R${}.", final_value);
        }
        3 => {
            println!("Você pagará R${} utilizando Débito.", valor);
            println!("Não haverá alteração do valor.");
            println!("O valor final será de R${}.", valor);
        }
        4 => {
            println!("Você pagará R${} utilizando Crédito.", valor);
            let final_valor = valor * 1.05;
            println!("Você irá pagar 5% de juros.");
            println!("O valor final será de R${}.", final_valor);
            println!("Você deseja parcelar sua compra de quantas vezes?");
            let mut parcelas = String::new();
            io::stdin().read_line(&mut parcelas)
                .expect("Input incorreto.");
            let parcelas: f64 = parcelas.trim()
                .parse()
                .expect("O valor não é numérico.");
                println!("Você pagará R${} em {} parcelas.", final_valor, parcelas);
                println!("O valor final será de R${} com {}% de juros de parcelamento sobre o valor total.", final_valor, parcelas*5.0);
                let final_value = final_valor * (parcelas*0.05);
                println!("O valor final será de R${:.2}.", final_value);
                println!("O valor de cada parcela é de R${:.2}.", final_value/parcelas);
                println!("Obrigado por comprar conosco!");
        }
        5 => {
            println!("Você pagará R${} utilizando Vale.", valor);
            let final_value = valor * 0.95;
            println!("Você receberá 5% de desconto.");
            println!("O valor final será de R${}.", final_value);
        }
        _ => { println!("Forma de pagamento inválida."); }
    }
}
