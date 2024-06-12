use crate::grafo_rs::{Arista, AristaT, NoPeso};

use super::Emparejamiento;

#[test]
fn test_creacion_emparejamiento()
{
    let aristas: Vec<Arista<i32, NoPeso>> = vec![Arista::arista_sin_peso(1, 2),
        Arista::arista_sin_peso(3, 6), Arista::arista_sin_peso(5, 4)];
    let mut emp1 
        = Emparejamiento::new(aristas.iter().collect::<Vec<&Arista<i32, NoPeso>>>())
        .expect("El emparejamiento es valido");

    assert!(emp1.size() == 3);
    assert!(!emp1.es_libre(&4));
    assert!(emp1.es_libre(&10));
    assert!(*emp1.buscar_arista(&1).expect("La arista esta en el emparejamiento") == aristas[0]);
    emp1.quitar_arista(&aristas[2]);
    assert!(emp1.es_libre(&4));

    assert!(emp1.recorrer(&6).expect("El vertice 6 esta saturado") == &3);
}

#[test]
fn test_creacion_emparejamiento_2()
{
    let aristas: Vec<Arista<i32, NoPeso>> = vec![Arista::arista_sin_peso(1, 2),
        Arista::arista_sin_peso(2, 3), Arista::arista_sin_peso(4, 5)];
    
    assert!(Emparejamiento::new(aristas.iter().collect::<Vec<&Arista<i32, NoPeso>>>()).is_none(),
        "El emparejamiento no es valido");
    
    let aristas: Vec<Arista<i32, NoPeso>> = vec![Arista::vertice(10), Arista::arista_sin_peso(1, 2),
        Arista::arista_sin_peso(3, 6), Arista::arista_sin_peso(5, 4)];
    
    assert!(Emparejamiento::new(aristas.iter().collect::<Vec<&Arista<i32, NoPeso>>>()).is_none(),
        "Contiene un vertice aislado");
}
