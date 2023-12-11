mod backtracking;
mod gerador;
mod programacao_dinamica;

use gerador::geracao_de_rotas;

fn main() {
    let rotas = geracao_de_rotas(5, 10, 0.5);
    println!("{rotas:?}");
}
