mod backtracking;
mod gerador;
mod programacao_dinamica;

use std::io;

fn main() {
    println!("Escolha o que quer executar:");
    println!("1 - Backtracking");
    println!("2 - Programação Dinâmica");
    println!("0 - Sair");
    loop {
        let mut input = String::new();
        println!("Digite a opção desejada:");
        if io::stdin().read_line(&mut input).is_err() {
            println!("Erro ao ler entrada");
            continue;
        }
        if let Ok(x) = input.trim().parse::<u8>() {
            match x {
                1 => {
                    backtracking::backtracking();
                }
                2 => {
                    programacao_dinamica::programacao_dinamica();
                }
                0 => {
                    println!("Saindo...");
                    break;
                }
                _ => {
                    println!("Entrada inválida");
                }
            }
        } else {
            println!("Entrada inválida");
        }
    }
}
