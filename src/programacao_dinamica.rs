pub fn programacao_dinamica(rotas: &[i32], caminhoes: usize) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = vec![vec![]; caminhoes];

    let mut rotas = rotas.to_vec();
    rotas.sort();
    rotas.reverse();

    let mut soma: Vec<i32> = vec![0; caminhoes];

    for rota in rotas {
        let mut index = 0;
        let mut menor = soma[0];

        for i in 1..caminhoes {
            if soma[i] < menor {
                menor = soma[i];
                index = i;
            }
        }

        soma[index] += rota;
        result[index].push(rota);
    }

    result
}
