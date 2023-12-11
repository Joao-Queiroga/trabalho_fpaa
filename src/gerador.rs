use rand::Rng;
use std::vec::Vec;

pub fn geracao_de_rotas(quant_rotas: i32, tam_conjunto: i32, dispersao: f64) -> Vec<Vec<i32>> {
    let tam_base: i32 = 13;
    let tam_max: i32 = (tam_base as f64 * (1.0 + dispersao)) as i32;

    let mut rng = rand::thread_rng();

    let conjunto_de_teste: Vec<Vec<i32>> = (0..tam_conjunto)
        .map(|_| {
            (0..quant_rotas)
                .map(|_| rng.gen_range(tam_base..tam_max))
                .collect()
        })
        .collect();

    conjunto_de_teste
}
