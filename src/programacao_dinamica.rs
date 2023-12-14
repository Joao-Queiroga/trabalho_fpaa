// Criar um código de programação dinâmica que Distribui necessariamente todas as rotas, que são os i32 marcando a distancia das rotas, para os caminhões de modo a fazer com que todos os caminhões percorram
// uma distância igual ou a mais próxima possível, de modo em que
// todas as rotas sejam designadas a um caminhão mesmo que os caminhões percorram mais de uma rota.
// O conjunto de rotas é passado como &[i32] em que o valor de cada i32 é a distancia da rota
// O número de caminhões é passado como usize
// O retorno deve ser um Vec<Vec<i32>> em que cada Vec<i32> representa as rotas designadas a um caminhão
pub fn programacao_dinamica(rotas: &[i32], n_caminhoes: usize) -> Vec<Vec<i32>> {
    let mut rotas = rotas.to_vec();
    let total: i32 = rotas.iter().sum();
    let n_rotas = rotas.len();
    let alvo = total / n_caminhoes as i32;
    let margem = alvo / 10;
    let mut caminhoes: Vec<Vec<i32>> = vec![vec![]; n_caminhoes];

    for caminhao in &mut caminhoes {
        let mut pd = vec![vec![false; (alvo + margem + 1) as usize]; n_rotas + 1];
        let mut escolhas = vec![vec![std::i32::MAX; (alvo + margem + 1) as usize]; n_rotas + 1];

        pd.iter_mut().for_each(|linha| linha[0] = true);

        for i in 1..=n_rotas {
            for j in 0..=(alvo + margem) as usize {
                if j < rotas[i - 1] as usize || !pd[i - 1][j - rotas[i - 1] as usize] {
                    pd[i][j] = pd[i - 1][j];
                } else {
                    pd[i][j] = true;
                    escolhas[i][j] = i as i32;
                }
            }
        }

        let melhor_soma = (alvo..=(alvo + margem))
            .find(|&j| pd[n_rotas][j as usize])
            .unwrap_or(0);

        let mut i = n_rotas;
        let mut j = melhor_soma;

        while i > 0 && j > 0 {
            if escolhas[i][j as usize] != 0 && rotas[i - 1] != 0 {
                caminhao.push(rotas[i - 1]);
                j -= rotas[i - 1];
                rotas[i - 1] = 0;
            }
            i -= 1;
        }
    }

    caminhoes[n_caminhoes - 1].extend(rotas.iter().filter(|&rota| *rota != 0));
    caminhoes
}
