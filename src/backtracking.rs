use super::gerador::gerador_de_rotas;
use std::time::{Duration, Instant};

fn distribuir_rotas(
    rotas: &[i32],
    num_caminhoes: usize,
    caminhao_atual: usize,
    distribuicao_atual: &mut Vec<Vec<i32>>,
    melhores_distribuicoes: &mut Vec<Vec<Vec<i32>>>,
) {
    if caminhao_atual == num_caminhoes {
        melhores_distribuicoes.push(distribuicao_atual.to_vec());
        return;
    }

    for (i, &rota) in rotas.iter().enumerate() {
        distribuicao_atual[caminhao_atual].push(rota);

        distribuir_rotas(
            &rotas[i + 1..],
            num_caminhoes,
            caminhao_atual + 1,
            distribuicao_atual,
            melhores_distribuicoes,
        );

        distribuicao_atual[caminhao_atual].pop();
    }
}

pub fn encontrar_melhor_distribuicao(rotas: &[i32], num_caminhoes: usize) -> Vec<Vec<Vec<i32>>> {
    let mut melhores_distribuicoes = Vec::new();
    let mut distribuicao_atual = vec![vec![]; num_caminhoes];

    distribuir_rotas(
        rotas,
        num_caminhoes,
        0,
        &mut distribuicao_atual,
        &mut melhores_distribuicoes,
    );

    melhores_distribuicoes
}

pub fn testar_tempo_execucao() -> usize {
    let mut quant_rotas: i32 = 6;
    let tempo_limite = Duration::from_secs(30);

    loop {
        let mut tempos_execucao = Vec::new();

        for rotas in gerador_de_rotas(quant_rotas, 10, 0.5) {
            let inicio = Instant::now();
            let _melhor_distribuicao = encontrar_melhor_distribuicao(&rotas, 3);
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
