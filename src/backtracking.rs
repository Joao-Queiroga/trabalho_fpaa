pub fn backtracking() {
    println!("Executando Backtracking");
    println!("Tamanho máximo do conjunto: {}", testar_tempo_execucao());
}

use super::gerador::gerador_de_rotas;
use std::time::{Duration, Instant};

fn calcular_total(rota: &[i32]) -> i32 {
    rota.iter().sum()
}

fn distribuir_rotas(
    rotas: &[i32],
    num_caminhoes: usize,
    caminhao_atual: usize,
    distribuicao_atual: &mut Vec<Vec<i32>>,
    melhores_distribuicoes: &mut Vec<Vec<Vec<i32>>>,
) {
    if caminhao_atual == num_caminhoes {
        let mut metrica_atual = 0;
        for i in 1..num_caminhoes {
            let diferenca = (calcular_total(&distribuicao_atual[i - 1])
                - calcular_total(&distribuicao_atual[i]))
            .abs();
            metrica_atual += diferenca;
        }

        let mut melhor_metrica = std::i32::MAX;
        for distribuicao in melhores_distribuicoes.iter() {
            let mut metrica = 0;
            for i in 1..num_caminhoes {
                let diferenca =
                    (calcular_total(&distribuicao[i - 1]) - calcular_total(&distribuicao[i])).abs();
                metrica += diferenca;
            }
            melhor_metrica = melhor_metrica.min(metrica);
        }

        // poda
        if metrica_atual <= melhor_metrica {
            melhores_distribuicoes.push(distribuicao_atual.clone());
        }
        return;
    }

    for rota in rotas.iter().cloned() {
        distribuicao_atual[caminhao_atual].push(rota);
        distribuir_rotas(
            rotas,
            num_caminhoes,
            caminhao_atual + 1,
            distribuicao_atual,
            melhores_distribuicoes,
        );
        distribuicao_atual[caminhao_atual].pop();
    }
}

fn encontrar_melhor_distribuicao(rotas: Vec<Vec<i32>>, num_caminhoes: usize) -> Vec<Vec<Vec<i32>>> {
    let rotas_flattened: Vec<i32> = rotas.into_iter().flatten().collect();
    let mut melhores_distribuicoes = Vec::new();
    let mut distribuicao_atual = vec![vec![]; num_caminhoes];

    distribuir_rotas(
        &rotas_flattened,
        num_caminhoes,
        0,
        &mut distribuicao_atual,
        &mut melhores_distribuicoes,
    );

    melhores_distribuicoes
}

fn testar_tempo_execucao() -> usize {
    let mut tamanho_conjunto: usize = 6;
    let tempo_limite = Duration::from_secs(30);

    loop {
        let mut tempos_execucao = Vec::new();

        for _ in 0..10 {
            let rotas_aleatorias: Vec<Vec<i32>> = gerador_de_rotas(3, tamanho_conjunto, 0.5);

            let inicio = Instant::now();
            let _melhor_distribuicao = encontrar_melhor_distribuicao(rotas_aleatorias, 3);
            let duracao = inicio.elapsed();

            tempos_execucao.push(duracao);
        }

        let tempo_medio: Duration =
            tempos_execucao.iter().sum::<Duration>() / tempos_execucao.len() as u32;

        if tempo_medio >= tempo_limite {
            break;
        }

        tamanho_conjunto += 1;
    }

    tamanho_conjunto
}
