use std::io;

fn calculadora(numero1: i32, numero2: i32, operacao: String) -> i32 {
    if operacao == "+" {
        numero1 + numero2 
    } else if operacao == "-" {
        numero1 - numero2      
    }else if operacao == "*" {
        numero1 * numero2
    }else if operacao == "/" {
        if numero2 == 0{
            println!("O divisor não pode ser zero");
            0
        }else {
            numero1 / numero2
        }
    }
    else {
        println!("Operação inválida!");
        0
    }
}


fn main() {

    let mut numero1 = String::new();
    let mut numero2 = String::new();
    let mut operacao = String::new();


    println!("Bem vindo ao app de calculadora feita em Rust");

    println!("Digite o primeiro numero: ");

    io::stdin()
        .read_line(&mut numero1)
        .expect("Falha ao ler a linha");

     let numero1: i32 = numero1.trim().parse().expect("Digite um numero valido");


    println!("Digite o segundo numero: ");

    io::stdin()
        .read_line(&mut numero2)
        .expect("Falha ao ler a linha");

    let numero2: i32 = numero2.trim().parse().expect("Digite um numero valido");

    println!("Por fim, digite a operação desejada (+, -, *, /): ");

    io::stdin()
    .read_line(&mut operacao)
    .expect("Falha ao ler a linha");

    let operacao = operacao.trim().to_string();

    println!("Resultado: {}", calculadora(numero1, numero2, operacao));


}
