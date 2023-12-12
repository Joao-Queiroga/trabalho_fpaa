mod backtracking;
mod gerador;
mod programacao_dinamica;

fn main() {
    use std::io;
    loop {
        println!("Opções:");
        println!("1 - Backtracking");
        println!("2 - Programação Dinâmica");
        println!("0 - Sair");
        println!("Digite a opção desejada:");
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Erro ao ler entrada");
            continue;
        }

        match input.trim().parse::<u8>() {
            Ok(1) => {
                backtracking::testar_tempo_execucao();
            }
            Ok(2) => programacao_dinamica::programacao_dinamica(),
            Ok(0) => {
                println!("Saindo...");
                break;
            }
            _ => println!("Entrada inválida"),
        }
    }
}

pub fn mostrar_distribuicao(distribuicao: &[Vec<i32>]) {
    println!("{distribuicao:?}")
}
