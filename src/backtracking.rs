pub fn backtracking(rotas: &[i32], n_caminhoes: usize) -> Vec<Vec<i32>> {
    let mut melhor_distribuicao = vec![vec![]; n_caminhoes];
    let mut melhor_distancia = 0;

    fn backtracking_recursivo(
        rotas: &[i32],
        distribuicao_atual: &mut Vec<Vec<i32>>,
        melhor_distribuicao: &mut Vec<Vec<i32>>,
        melhor_distancia: &mut i32,
        caminhao_atual: usize,
    ) {
        if rotas.is_empty() {
            let distancia_total: i32 = distribuicao_atual
                .iter()
                .map(|caminhao| caminhao.iter().sum::<i32>())
                .sum();

            if *melhor_distancia == 0 || distancia_total < *melhor_distancia {
                *melhor_distancia = distancia_total;
                *melhor_distribuicao = distribuicao_atual.clone();
            }
            return;
        }

        for _ in 0..distribuicao_atual.len() {
            distribuicao_atual[caminhao_atual].push(rotas[0]);
            backtracking_recursivo(
                &rotas[1..],
                distribuicao_atual,
                melhor_distribuicao,
                melhor_distancia,
                (caminhao_atual + 1) % distribuicao_atual.len(),
            );
            distribuicao_atual[caminhao_atual].pop();
        }
    }

    let mut distribuicao_atual = vec![vec![]; n_caminhoes];
    backtracking_recursivo(
        rotas,
        &mut distribuicao_atual,
        &mut melhor_distribuicao,
        &mut melhor_distancia,
        0,
    );

    melhor_distribuicao
}

pub fn testar_tempo_execucao() -> usize {
    use super::gerador::gerador_de_rotas;
    use std::time::{Duration, Instant};

    let mut quant_rotas: i32 = 6;
    let tempo_limite = Duration::from_secs(30);

    loop {
        let mut tempos_execucao = Vec::new();

        for rotas in gerador_de_rotas(quant_rotas, 10, 0.5) {
            let inicio = Instant::now();
            let _melhor_distribuicao = backtracking(&rotas, 3);
            let duracao = inicio.elapsed();

            tempos_execucao.push(duracao);
        }

        let tempo_medio: Duration =
            tempos_execucao.iter().sum::<Duration>() / tempos_execucao.len() as u32;

        if tempo_medio >= tempo_limite {
            break;
        }

        quant_rotas += 1;
    }

    quant_rotas as usize
}
