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

#[test]
fn test_maximizar_flujo_2()
{
    let arco1 = Diarista::arista_sin_peso('1', '2');
    let arco2 = Diarista::arista_sin_peso('2', '1');
    let arco3 = Diarista::arista_sin_peso('1', '3');
    let arco4 = Diarista::arista_sin_peso('3', '2');
    let arco5 = Diarista::arista_sin_peso('2', '4');
    let arco6 = Diarista::arista_sin_peso('4', '3');
    
    let mut red: Red<char, NoPeso> = Red::new(None, 's', [('1', 16), ('2', 13)].to_vec(), 
        't', [('3', 20), ('4', 4)].to_vec(),
        [(arco1.clone(), 10), (arco2.clone(), 4), (arco3.clone(), 12), (arco4.clone(), 9),
        (arco5.clone(), 14), (arco6.clone(), 7)].to_vec()).expect("La red debe poder construirse");
    maximizar_flujo(&mut red);
    assert!(red.get_valor_red() == 23);
}

#[test]
fn test_maximizar_flujo_3()
{
    let arco1 = Diarista::arista_sin_peso('A', 'M');
    let arco2 = Diarista::arista_sin_peso('A', 'I');
    let arco3 = Diarista::arista_sin_peso('B', 'I');
    let arco4 = Diarista::arista_sin_peso('C', 'I');
    let arco5 = Diarista::arista_sin_peso('C', 'N');
    let arco6 = Diarista::arista_sin_peso('I', 'i');
    let arco7 = Diarista::arista_sin_peso('i', 'M');
    let arco8 = Diarista::arista_sin_peso('i', 'N');

    let mut red: Red<char, NoPeso> = Red::new(None, 's', [('A', 1000), ('B', 1000), ('C', 1000)].to_vec(), 
        't', [('M', 1000), ('N', 1000)].to_vec(), [(arco1.clone(), 30),
            (arco2.clone(), 40), (arco3.clone(), 30), (arco4.clone(), 80), (arco5.clone(), 20), (arco6.clone(), 70),
            (arco7.clone(), 60), (arco8.clone(), 50)].to_vec()).expect("La red debe poder construirse");
    maximizar_flujo(&mut red);

    assert!(red.get_valor_red() == 120);
}

#[test]
fn test_maximizar_flujo_4()
{
    let arco1 = Diarista::arista_sin_peso('b', 'a');
    let arco2 = Diarista::arista_sin_peso('a', 'e');
    let arco3 = Diarista::arista_sin_peso('a', 'c');
    let arco4 = Diarista::arista_sin_peso('c', 'b');
    let arco5 = Diarista::arista_sin_peso('c', 'e');
    let arco6 = Diarista::arista_sin_peso('b', 'd');
    let arco7 = Diarista::arista_sin_peso('d', 'e');

    let mut red: Red<char, NoPeso> = Red::new(None, 's', [('a', 4), ('b', 6)].to_vec(), 
        't', [('a', 5), ('e', 5)].to_vec(), [(arco1.clone(), 2),
            (arco2.clone(), 1), (arco3.clone(), 5), (arco4.clone(), 3), (arco5.clone(), 5), (arco6.clone(), 3), (arco7.clone(), 2)].to_vec())
        .expect("La red debe poder construirse");
    maximizar_flujo(&mut red);

    assert!(red.get_valor_red() == 8);
}
