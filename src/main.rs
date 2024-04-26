use grafo_rs::grafo_rs::Grafo;
use grafo_rs::grafo_rs::Arista;
use grafo_rs::grafo_rs::NoPeso;
use grafo_rs::grafo_rs::PesoT;

use grafo_rs::grafo_rs::arbol_camino_minimo;
use grafo_rs::grafo_rs::arbol_peso_minimo;
use grafo_rs::grafo_rs::arbol_profundidad;
use grafo_rs::grafo_rs::Etiquetado;

#[test]
fn test_adicion_aristas()
{
    let mut grafo0: Grafo<i32, NoPeso> = Grafo::from_aristas([Arista::arista_sin_peso(1, 2),
                                                                Arista::arista_sin_peso(2, 3),
                                                                Arista::arista_sin_peso(3, 4),
                                                                Arista::VerticeAislado(10),
                                                                Arista::arista_sin_peso(3, 2)].to_vec());
    assert_eq!(grafo0.get_aristas().len(), 4);
    grafo0.add_aristas(vec![Arista::arista_sin_peso(2, 3), Arista::arista_sin_peso(3, 2)]);
    assert_eq!(grafo0.get_aristas().len(), 4);
}

#[test]
fn test_eliminacion_aristas_vertices()
{
    let mut grafo0: Grafo<i32, NoPeso> = Grafo::from_aristas([Arista::arista_sin_peso(1, 2),
                                                                    Arista::arista_sin_peso(2, 3),
                                                                    Arista::arista_sin_peso(3, 4),
                                                                    Arista::VerticeAislado(10),
                                                                    Arista::arista_sin_peso(3, 2)].to_vec());

    assert_eq!(grafo0.grado(&10), Some(0), "Comprobar que el vertice 10 tiene grado 0");
    grafo0.remove_vertice(&10);
    assert_eq!(grafo0.grado(&10), None, "Comprobar que se haya eliminado");

    let mut grafo1 = grafo0.clone();

    grafo0.remove_arista(&Arista::arista_sin_peso(3, 4));
    assert_eq!(grafo0.grado(&3), Some(1));
    assert_eq!(grafo0.grado(&4), Some(0));

    grafo1.remove_vertice(&2);
    assert_eq!(grafo1.grado(&3), Some(1));
    assert_eq!(grafo1.grado(&1), Some(0));
    // println!("Lista de Vertices: {:?}", grafo1.get_vertices());
}

#[test]
fn test_grado()
{
    let grafo0: Grafo<i32, grafo_rs::grafo_rs::NoPeso> = Grafo::from_aristas([Arista::arista_sin_peso(1, 2),
                                                        Arista::arista_sin_peso(2, 3),
                                                        Arista::arista_sin_peso(3, 2),
                                                        Arista::VerticeAislado(5)].to_vec());
    
    assert_eq!(Some(0), grafo0.grado(&5));
    assert_eq!(Some(1), grafo0.grado(&1));
    assert_eq!(Some(2), grafo0.grado(&2));
    assert_eq!(None, grafo0.grado(&20));
}

#[test]
fn test_entorno()
{
    let grafo0: Grafo<i32, NoPeso> = Grafo::from_aristas([Arista::arista_sin_peso(1, 2),
                                                        Arista::arista_sin_peso(2, 3),
                                                        Arista::VerticeAislado(5)].to_vec());

    assert_eq!(vec![2], grafo0.entorno(&1).unwrap().into_iter().map(|x| *x).collect::<Vec<i32>>());
    assert_eq!(vec![1, 3], grafo0.entorno(&2).unwrap().into_iter().map(|x| *x).collect::<Vec<i32>>());
}

#[test]
fn test_get_vertices()
{
    let grafo0: Grafo<i32, NoPeso> = Grafo::from_aristas([Arista::arista_sin_peso(1, 2),
                                                        Arista::arista_sin_peso(2, 3),
                                                        Arista::arista_sin_peso(3, 2),
                                                        Arista::VerticeAislado(5)].to_vec());

    assert_eq!(vec![1, 2, 3, 5], grafo0.get_vertices().into_iter().map(|x| *x).collect::<Vec<i32>>());
}

#[test]
fn test_sucesion_grados()
{
    let grafo0: Grafo<i32, NoPeso> = Grafo::from_aristas([Arista::arista_sin_peso(1, 2),
                                                        Arista::arista_sin_peso(2, 3),
                                                        Arista::arista_sin_peso(3, 2),
                                                        Arista::VerticeAislado(5)].to_vec());

    assert_eq!(vec![2, 1, 1, 0], grafo0.sucesion_grados());
}

#[test]
fn test_sucesion_grafica()
{
    assert!(grafo_rs::grafo_rs::Grafo::<i32, NoPeso>::comprobar_sucesion(&vec![2, 1, 1]));
    assert!(!grafo_rs::grafo_rs::Grafo::<i32, NoPeso>::comprobar_sucesion(&vec![2, 1, 0]));
    assert!(grafo_rs::grafo_rs::Grafo::<i32, NoPeso>::comprobar_sucesion(&vec![4, 4, 3, 2, 2, 2, 1]));
}

#[test]
fn test_arbol_generador_minimo_1()
{
    let g: Grafo<i32, isize> = Grafo::from_aristas([Arista::arista(1, 2, Some(2)),
                                                         Arista::arista(2, 3, Some(5)),
                                                         Arista::arista(2, 4, Some(20)),
                                                         Arista::arista(1, 4, Some(1))   
                                                            ].to_vec());
    
    let arbol = arbol_peso_minimo(&g).expect("Arbol generador no producido").into_grafo();
    let entorno_1 = arbol.entorno(&1).expect("Entorno de 1 no existe");
    assert!(entorno_1.contains(&&2));
    assert!(entorno_1.contains(&&4));
    let entorno_2 = arbol.entorno(&&2).expect("Entorno de 2 no existe");
    assert!(entorno_2.contains(&&3));

    assert!(entorno_1.len() == 2);
    assert!(entorno_2.len() == 2);
    assert!(arbol.get_vertices().len() == 4);
}

#[test]
fn test_arbol_generador_minimo_2()
{
    let g: Grafo<i32, isize> = Grafo::from_aristas([Arista::arista(4, 5, Some(15)),
                                                        Arista::arista(1, 2, Some(7)),
                                                        Arista::arista(2, 3, Some(8)),
                                                        Arista::arista(3, 5, Some(5)),
                                                        Arista::arista(1, 4, Some(5)),
                                                        Arista::arista(2, 4, Some(9)),
                                                        Arista::arista(2, 5, Some(7)),
                                                        Arista::arista(4, 5, Some(15)),
                                                        Arista::arista(4, 6, Some(6)),
                                                        Arista::arista(5, 6, Some(8)),
                                                        Arista::arista(6, 7, Some(11)),
                                                        Arista::arista(5, 7, Some(9))].to_vec());
    let arbol = arbol_peso_minimo(&g).expect("El arbol no existe").into_grafo();
    
    let entorno_1 = arbol.entorno(&1).expect("El vertice 1 no esta en el arbol");
    assert!(entorno_1.len() == 2);
    let entorno_4 = arbol.entorno(&4).expect("El vertice 4 no esta en el arbol");
    assert!(entorno_4.len() == 2);
    let entorno_7 = arbol.entorno(&7).expect("El vertice 7 no esta en el arbol");
    assert!(entorno_7 == vec![&5]);
    let entorno_6 = arbol.entorno(&6).expect("El vertice 6 no esta en el arbol");
    assert!(entorno_6 == vec![&4]);
    
    // Comprobar el peso del arbol
    assert!(Arista::<i32, isize>::sumatorio_pesos(arbol.get_aristas()).expect("Una arista no tiene peso!!") == 39);
}

#[test]
fn test_arbol_profundidad_1()
{
    let g: Grafo<i32, NoPeso> = Grafo::from_aristas([Arista::arista(8, 6, None),
                                                        Arista::arista(6, 2, None),
                                                        Arista::arista(6, 3, None),
                                                        Arista::arista(2, 1, None),
                                                        Arista::arista(3, 1, None),
                                                        Arista::arista(2, 7, None)].to_vec());
    
    println!("El arbol de busqueda por profundidad no es unico. Se imprimira el arbol: ");
    let prof = arbol_profundidad(&g, &8).expect("El arbol debe existir").0.into_grafo();
    assert!(prof.size() == g.size());
    for arista in prof.get_aristas().into_iter()
    {
        if let Arista::Arista(v, w, _) = arista
        {
            println!("Arista: {} -> {}", v, w);
        }
        else 
        {
            panic!("No deben haber vertices aislados");
        }
    }
}

#[test]
fn test_arbol_profundidad_2()
{
    let g: Grafo<i32, NoPeso> = Grafo::from_aristas([Arista::arista(1, 2, None),
                                                            Arista::arista(1, 3, None),
                                                            Arista::arista(3, 4, None),
                                                            Arista::arista(3, 5, None),
                                                            Arista::arista(4, 5, None),
                                                            Arista::vertice(10)].to_vec());
    println!("El arbol de busqueda se imprimira:");
    let prof = arbol_profundidad(&g, &1).expect("El arbol debe existir").0.into_grafo();
    assert!(prof.size() == g.size() - 1);
    for arista in prof.get_aristas().into_iter()
    {
        if let Arista::Arista(v, w, _) = arista
        {
            println!("Arista: {} -> {}", v, w);
        }
        else
        {
            panic!("El arbol solo puede contener aristas");    
        }
    }
}

#[test]
fn test_arbol_profundidad_3()
{
    let g: Grafo<i32, NoPeso> = Grafo::from_aristas([Arista::arista(1, 2, None),
                                                            Arista::arista(2, 3, None),
                                                            Arista::arista(1, 10, None),
                                                            Arista::arista(3, 10, None),
                                                            Arista::arista(10, 4, None),
                                                            Arista::arista(4, 5, None),
                                                            Arista::arista(10, 5, None)].to_vec());
    
    let prof = arbol_profundidad(&g, &10).expect("El arbol debe existir").0.into_grafo();
    assert!(prof.size() == g.size());
    for arista in prof.get_aristas().into_iter()
    {
        if let Arista::Arista(v, w, _) = arista
        {
            println!("Arista: {} -> {}", v, w);
        }
        else 
        {
            panic!("El arbol solo puede contener aristas");    
        }
    }
}

#[test]
fn test_arbol_profundidad_4()
{
    let g: Grafo<char> = Grafo::from_aristas([Arista::arista('A', 'B', None),
                                                    Arista::arista('B', 'C', None),
                                                    Arista::arista('A', 'C', None),
                                                    Arista::arista('C', 'D', None),
                                                    Arista::arista('D', 'E', None),
                                                    Arista::arista('E', 'F', None),
                                                    Arista::arista('D', 'F', None),
                                                    Arista::arista('D', 'G', None),
                                                    Arista::arista('F', 'H', None),
                                                    Arista::arista('H', 'I', None),
                                                    Arista::arista('G', 'I', None),
                                                    Arista::arista('I', 'J', None),
                                                    Arista::arista('J', 'K', None),
                                                    Arista::arista('K', 'L', None),
                                                    Arista::arista('K', 'N', None),
                                                    Arista::arista('N', 'M', None),
                                                    Arista::arista('M', 'O', None),
                                                    Arista::arista('O', 'I', None),
                                                    Arista::arista('I', 'L', None)].to_vec());
    
    let prof = arbol_profundidad(&g, &'A').expect("El arbol debe existir").0.into_grafo();
    println!("Se imprimira el arbol");
    for arista in prof.get_aristas().into_iter()
    {
        if let Arista::Arista(v, w, _) = arista
        {
            println!("Arista: {} -> {}", v, w);
        }
        else 
        {
            panic!("Solo deben haber aristas"); 
        }
    }

}

#[test]
fn test_dijkstra()
{
    let g: Grafo<i32, isize> = Grafo::from_aristas([Arista::arista(1, 5, Some(9)),
                                                            Arista::arista(5, 2, Some(5)),
                                                            Arista::arista(2, 3, Some(3)),
                                                            Arista::arista(2, 6, Some(8)),
                                                            Arista::arista(3, 6, Some(2)),
                                                            Arista::arista(3, 4, Some(11)),
                                                            Arista::arista(6, 7, Some(5)),
                                                            Arista::arista(4, 7, Some(3))].to_vec());
    let min = arbol_camino_minimo(&g, &4).expect("El arbol debe existir");
    let distancias = min.1;
    assert_eq!(distancias.buscar_vertice(&4).unwrap().get_valor(), 0);
    assert_eq!(distancias.buscar_vertice(&3).unwrap().get_valor(), 10);
    assert_eq!(distancias.buscar_vertice(&2).unwrap().get_valor(), 13);
}

fn main() {}
