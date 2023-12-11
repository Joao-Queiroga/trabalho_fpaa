mod backtracking;
mod gerador;
mod programacao_dinamica;

fn main() {
    println!("Escolha o que quer executar:");
    println!("1 - Backtracking");
    println!("2 - Programação Dinâmica");
    println!("0 - Sair");

    use std::io;

    loop {
        println!("Digite a opção desejada:");
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Erro ao ler entrada");
            continue;
        }

        match input.trim().parse::<u8>() {
            Ok(1) => backtracking::backtracking(),
            Ok(2) => programacao_dinamica::programacao_dinamica(),
            Ok(0) => {
                println!("Saindo...");
                break;
            }
            _ => println!("Entrada inválida"),
        }
    }
}
