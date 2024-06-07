use super::Bipartido;

use crate::grafo_rs::{Arista, AristaT, Grafo, GrafoT, NoPeso};

#[test]
fn test_grafo_no_bipartido()
{
    let g: Grafo<i32, NoPeso> = Grafo::from_aristas([Arista::arista_sin_peso(1, 2), 
            Arista::arista_sin_peso(2, 3), Arista::arista_sin_peso(1, 3)].to_vec());
    
    assert!(Bipartido::from_grafo(&g).is_none());
}

#[test]
fn test_grafo_no_bipartido_2()
{
    let g: Grafo<i32, NoPeso> = Grafo::from_aristas([Arista::arista_sin_peso(1, 2),
        Arista::arista_sin_peso(1, 3), Arista::arista_sin_peso(1, 4), Arista::arista_sin_peso(2, 4),
        Arista::arista_sin_peso(2, 5), Arista::vertice(10)].to_vec());
    
    assert!(Bipartido::from_grafo(&g).is_none());
}

#[test]
fn test_grafo_bipartido()
{
    let g: Grafo<i32, NoPeso> = Grafo::from_aristas([Arista::arista_sin_peso(1, 2),
        Arista::arista_sin_peso(1, 3), Arista::arista_sin_peso(3, 4), Arista::arista_sin_peso(4, 5),
        Arista::arista_sin_peso(1, 5), Arista::VerticeAislado(10)].to_vec());
    
    let bipartido = Bipartido::from_grafo(&g).expect("El grafo es bipartido");
    let aristas = bipartido.get_aristas();
    assert!(aristas.contains(&Arista::arista_sin_peso(1, 2)));
    assert!(aristas.contains(&Arista::arista_sin_peso(4, 3)));
    assert!(!aristas.contains(&Arista::vertice(10)), "No deben haber vertices aislados");
    let aristas: Vec<(&i32, &i32)> = aristas.into_iter()
        .map(|x| x.get_vertices().expect("No pueden haber vertices aislados"))
        .collect();
    assert!(!aristas.contains(&(&3, &4)), "El orden no es correcto");
}

#[test]
fn test_grafo_bipartido_partes()
{
    let g: Grafo<i32, NoPeso> = Grafo::from_aristas([Arista::arista_sin_peso(1, 2),
    Arista::arista_sin_peso(1, 3), Arista::arista_sin_peso(3, 4), Arista::arista_sin_peso(4, 5),
    Arista::arista_sin_peso(1, 5), Arista::VerticeAislado(10)].to_vec());

    let bipartido = Bipartido::from_grafo(&g).expect("El grafo es bipartido");

    let vertices_esperados_x = vec![&1, &4];
    let vertices_esperados_y = vec![&2, &3, &5];

    assert!(bipartido.get_vertices_x() == vertices_esperados_x);
    assert!(bipartido.get_vertices_y() == vertices_esperados_y);
}
