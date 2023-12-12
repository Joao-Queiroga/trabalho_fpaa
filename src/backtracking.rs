// Distribui necessariamente todas as rotas, que são os i32 marcando a distancia das rotas, para os caminhões de modo a fazer com que todos os caminhões percorram
// uma distância igual ou a mais próxima possível usando backtracking com poda, de modo em que
// todas as rotas sejam designadas a um caminhão mesmo que os caminhões percorram mais de uma rota.
// O conjunto de rotas é passado como &[i32] em que o valor de cada i32 é a distancia da rota
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

        for i in 0..distribuicao_atual.len() {
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
