pub fn backtracking(rotas: &[i32], n_caminhoes: usize) -> Vec<Vec<i32>> {
    let mut melhor_distribuicao = vec![vec![]; n_caminhoes];
    let mut melhor_distancia = 0;

    fn backtracking_recursivo(
        rotas: &[i32],
        distribuicao_atual: &mut Vec<Vec<i32>>,
        melhor_distribuicao: &mut Vec<Vec<i32>>,
        melhor_distancia: &mut i32,
        caminhao_atual: usize,
        melhor_diferenca: &mut i32, // Novo parâmetro para controlar a melhor diferença
    ) {
        if rotas.is_empty() {
            let mut min = std::i32::MAX;
            let mut max = std::i32::MIN;

            for caminhao in distribuicao_atual.iter() {
                let distancia = caminhao.iter().sum::<i32>();
                if distancia < min {
                    min = distancia;
                }
                if distancia > max {
                    max = distancia;
                }
            }

            let diferenca_atual = max - min;
            if diferenca_atual >= *melhor_diferenca {
                return; // Podar o ramo atual se a diferença for maior ou igual à melhor encontrada
            }

            let distancia_total: i32 = distribuicao_atual
                .iter()
                .map(|caminhao| caminhao.iter().sum::<i32>())
                .sum();

            if *melhor_distancia == 0 || distancia_total < *melhor_distancia {
                *melhor_distancia = distancia_total;
                *melhor_diferenca = diferenca_atual; // Atualizar a melhor diferença
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
                melhor_diferenca, // Passar a referência da melhor diferença para a chamada recursiva
            );
            distribuicao_atual[caminhao_atual].pop();
        }
    }

    let mut distribuicao_atual = vec![vec![]; n_caminhoes];
    let mut melhor_diferenca = std::i32::MAX; // Inicializar a melhor diferença com o maior valor possível
    backtracking_recursivo(
        rotas,
        &mut distribuicao_atual,
        &mut melhor_distribuicao,
        &mut melhor_distancia,
        0,
        &mut melhor_diferenca,
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
