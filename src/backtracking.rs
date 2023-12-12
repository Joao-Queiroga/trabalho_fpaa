use super::gerador::gerador_de_rotas;
use std::time::{Duration, Instant};

fn calcular_total(rota: &[i32]) -> i32 {
    rota.iter().sum()
}

fn distribuir_rotas_recursivo(
    rotas: &[i32],
    num_caminhoes: usize,
    caminhao_atual: usize,
    distribuicao_atual: &mut Vec<Vec<i32>>,
    melhores_distribuicoes: &mut Vec<Vec<Vec<i32>>>,
) {
    if caminhao_atual == num_caminhoes {
        if distribuicao_atual.iter().all(|caminhao| {
            calcular_total(caminhao) == calcular_total(rotas) / num_caminhoes as i32
        }) {
            let mut metrica_atual = 0;
            for i in 1..num_caminhoes {
                if i < distribuicao_atual.len() {
                    let diferenca = (calcular_total(&distribuicao_atual[i - 1])
                        - calcular_total(&distribuicao_atual[i]))
                    .abs();
                    metrica_atual += diferenca;
                }
            }

            let mut melhor_metrica = std::i32::MAX;
            for distribuicao in melhores_distribuicoes.iter() {
                let mut metrica = 0;
                for i in 1..num_caminhoes {
                    if i < distribuicao.len() {
                        let diferenca = (calcular_total(&distribuicao[i - 1])
                            - calcular_total(&distribuicao[i]))
                        .abs();
                        metrica += diferenca;
                    }
                }
                melhor_metrica = melhor_metrica.min(metrica);
            }

            // Poda
            if metrica_atual <= melhor_metrica {
                let distribuicao_clone = distribuicao_atual.to_vec();
                melhores_distribuicoes.push(distribuicao_clone);
            }
            return;
        }
    }

    for &rota in rotas.iter() {
        if caminhao_atual < distribuicao_atual.len()
            && distribuicao_atual[caminhao_atual].iter().sum::<i32>() + rota
                <= calcular_total(rotas) / num_caminhoes as i32
        {
            distribuicao_atual[caminhao_atual].push(rota);
            distribuir_rotas_recursivo(
                rotas,
                num_caminhoes,
                caminhao_atual + 1,
                distribuicao_atual,
                melhores_distribuicoes,
            );
            distribuicao_atual[caminhao_atual].pop();
        }
    }
}

pub fn distribuir_rotas(rotas: &[i32], num_caminhoes: usize) -> Vec<Vec<Vec<i32>>> {
    let mut melhores_distribuicoes = Vec::new();
    let mut distribuicao_atual = vec![vec![]; num_caminhoes];

    distribuir_rotas_recursivo(
        rotas,
        num_caminhoes,
        0,
        &mut distribuicao_atual,
        &mut melhores_distribuicoes,
    );

    melhores_distribuicoes
}

pub fn encontrar_melhor_distribuicao(rotas: &[i32], num_caminhoes: usize) -> Vec<Vec<Vec<i32>>> {
    distribuir_rotas(rotas, num_caminhoes)
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
