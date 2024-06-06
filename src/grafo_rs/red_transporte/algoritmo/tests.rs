use super::encontrar_camino_aumento;

use crate::{grafo_rs::{AristaT, Diarista, NoPeso, Red}, incrementar_flujo};

#[test]
fn test_camino_aumento()
{
    let mut red: Red<i32, NoPeso> = Red::new(None, 1, [(2, 4), (3, 6)].to_vec(), 
        7, [(5, 1), (6, 2)].to_vec(), [(Diarista::arista_sin_peso(3, 5), 1)].to_vec())
        .expect("La red debe poder crearse");

    let camino1 = encontrar_camino_aumento(&red).expect("Debe existir un camino de aumento");
    let camino_esperado: Vec<Diarista<i32, NoPeso>> = vec![Diarista::arista_sin_peso(1, 3), Diarista::arista_sin_peso(3, 5), Diarista::arista_sin_peso(5, 7)];
    assert!(camino1 == camino_esperado.iter().collect::<Vec<&Diarista<i32, NoPeso>>>());

    incrementar_flujo!(red, &Diarista::arista_sin_peso(3, 5));
    let camino2 = encontrar_camino_aumento(&red);
    assert!(camino2.is_none(), "Hemos saturado el arco (3,5)");
}
