use crate::{grafo_rs::{AristaT, Diarista, NoPeso}, incrementar_flujo};

use super::Red;

#[test]
fn test_comparacion_flujos()
{
    let arco1: Diarista<i32, NoPeso> = Diarista::arista_sin_peso(3, 4);
    let arco2: Diarista<i32, NoPeso> = Diarista::arista_sin_peso(4, 8);
    let arco3: Diarista<i32, NoPeso> = Diarista::arista_sin_peso(4, 9);
    let arco4: Diarista<i32, NoPeso> = Diarista::arista_sin_peso(4, 3);
    let mut red: Red<i32, NoPeso> = Red::new(None, 1, [(2, 1), (3, 1)].to_vec(),
            10, [(9, 2), (8, 2)].to_vec(), 
            [(arco1.clone(), 1), (arco2.clone(), 2), (arco3.clone(), 3), (arco4.clone(), 7)].to_vec());

    red.set_valor(&arco1, 8);
    assert!(red.get_valor(&arco1).expect("El flujo existe") == 0);
    red.set_valor(&arco1, 1);
    assert!(red.get_valor(&arco1).unwrap() == 1);

    red.set_valor(&arco4, 6);
    assert!(red.get_valor(&arco4).expect("El flujo existe") == 6);

    let flujo1 = red.get_flujo(&arco1).unwrap();
    let flujo2 = red.get_flujo(&arco4).unwrap();
    assert!(flujo1 < flujo2);

    red.set_valor(&arco4, 0);
    let flujo1 = red.get_flujo(&arco1).unwrap();
    let flujo2 = red.get_flujo(&arco4).unwrap();
    assert!(flujo1 > flujo2);
}

#[test]
fn test_creacion_red()
{
    let arco1: Diarista<i32, NoPeso> = Diarista::arista_sin_peso(1, 2);
    let red = Red::new(None, 1, [(2, 2)].to_vec(), 
        2, [(1, 4)].to_vec(), vec![(arco1.clone(), 10)]);
    assert!(red.get_flujo(&arco1).expect("arco1 no esta en la red").get_capacidad() == 2, 
        "La funcion get_valor devuelve el primer flujo");
}

#[test]
fn test_incremento_flujo()
{
    let arco1: Diarista<i32, NoPeso> = Diarista::arista_sin_peso(3, 4);
    let arco2: Diarista<i32, NoPeso> = Diarista::arista_sin_peso(4, 8);
    let arco3: Diarista<i32, NoPeso> = Diarista::arista_sin_peso(4, 9);
    let arco4: Diarista<i32, NoPeso> = Diarista::arista_sin_peso(4, 3);
    let mut red: Red<i32, NoPeso> = Red::new(None, 1, [(2, 1), (3, 1)].to_vec(),
            10, [(9, 2), (8, 2)].to_vec(), 
            [(arco1.clone(), 1), (arco2.clone(), 2), (arco3.clone(), 3), (arco4.clone(), 7)].to_vec());

    incrementar_flujo!(red, &arco1).expect("Debe ser posible incrementar el flujo");
    assert!(red.get_valor(&arco1).unwrap() == 1);
    assert!(incrementar_flujo!(red, &arco1) == None, "Ha alcanzado el limite de capacidad");

    assert!(incrementar_flujo!(red, &arco2, 10) == None, "Es superior a la capacidad");
    assert!(red.get_valor(&arco2).unwrap() == 0);
    incrementar_flujo!(red, &arco2, 2).expect("Debe ser posible incrementar el valor");
}
