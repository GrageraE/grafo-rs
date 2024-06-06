use super::{encontrar_camino_aumento, maximizar_flujo};

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

#[test]
fn test_camino_aumento_2()
{
    let arco1 = Diarista::arista_sin_peso('a', 'b');
    let arco2 = Diarista::arista_sin_peso('a', 'c');
    let arco3 = Diarista::arista_sin_peso('b', 'c');
    let arco4 = Diarista::arista_sin_peso('c', 'd');
    let arco5 = Diarista::arista_sin_peso('d', 'b');
    let red: Red<char, NoPeso> = Red::new(None, 's', [('a', 5), ('c', 5)].to_vec(), 
        't', [('b', 8), ('d', 3)].to_vec(), [(arco1.clone(), 3), (arco2.clone(), 6),
            (arco3.clone(), 8), (arco4.clone(), 6), (arco5.clone(), 3)].to_vec())
        .expect("La red debe poder crearse");
    let camino1 = encontrar_camino_aumento(&red).expect("El camino debe existir");
    let camino_esperado: Vec<Diarista<char, NoPeso>> = vec![Diarista::arista_sin_peso('s', 'a'),
            Diarista::arista_sin_peso('a', 'b'), Diarista::arista_sin_peso('b', 't')];
    assert!(camino1 == camino_esperado.iter().collect::<Vec<&Diarista<char, NoPeso>>>());
}

#[test]
fn test_maximizar_flujo()
{
    let arco1 = Diarista::arista_sin_peso('a', 'b');
    let arco2 = Diarista::arista_sin_peso('a', 'c');
    let arco3 = Diarista::arista_sin_peso('b', 'c');
    let arco4 = Diarista::arista_sin_peso('c', 'd');
    let arco5 = Diarista::arista_sin_peso('d', 'b');
    let mut red: Red<char, NoPeso> = Red::new(None, 's', [('a', 5), ('c', 5)].to_vec(), 
        't', [('b', 8), ('d', 3)].to_vec(), [(arco1.clone(), 3), (arco2.clone(), 6),
            (arco3.clone(), 8), (arco4.clone(), 6), (arco5.clone(), 3)].to_vec())
        .expect("La red debe poder crearse");

    maximizar_flujo(&mut red);
    assert!(red.get_valor_red() == 9, "El flujo maximal debe ser 9");
    assert!(red.get_valor(&Diarista::arista_sin_peso('s', 'a')) == Some(4));
    assert!(red.get_valor(&Diarista::arista_sin_peso('s', 'c')) == Some(5));
    assert!(red.get_valor(&arco1) == Some(3));
}
