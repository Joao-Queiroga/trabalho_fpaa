use rand::Rng;
use std::vec::Vec;

pub fn gerador_de_rotas(quant_rotas: i32, tam_conjunto: usize, dispersao: f64) -> Vec<Vec<i32>> {
    let tam_base = 13;
    let tam_max = (tam_base as f64 * (1.0 + dispersao)) as i32;

    let mut rng = rand::thread_rng();

    (0..tam_conjunto)
        .map(|_| {
            (0..quant_rotas)
                .map(|_| rng.gen_range(tam_base..tam_max))
                .collect()
        })
        .collect()
}
