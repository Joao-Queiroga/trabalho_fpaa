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

        let mut melhor_soma = (alvo..=(alvo + margem))
            .find(|&j| pd[n_rotas][j as usize])
            .unwrap_or(0);

        for i in (1..=n_rotas).rev() {
            if escolhas[i][melhor_soma as usize] != 0 && rotas[i - 1] != 0 {
                caminhao.push(rotas[i - 1]);
                melhor_soma -= rotas[i - 1];
                rotas[i - 1] = 0;
            }
            if melhor_soma <= 0 {
                break;
            };
        }
    }

    caminhoes[n_caminhoes - 1].extend(rotas.iter().filter(|&rota| *rota != 0));
    caminhoes
}
