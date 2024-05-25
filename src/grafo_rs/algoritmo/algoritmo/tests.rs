#[cfg(test)]
mod tests
{
    use core::panic;

    use super::super::{Arista, Grafo};
    use crate::grafo_rs::{AristaT, Diarista, Digrafo, GrafoT, NoPeso};
    use super::super::{arbol_camino_minimo, arbol_peso_minimo, 
                    arbol_profundidad, comprobar_sucesion, arbol_anchura};

    #[test]
    fn test_sucesion_grafica()
    {
        assert!(comprobar_sucesion(&vec![2, 1, 1]));
        assert!(!comprobar_sucesion(&vec![2, 1, 0]));
        assert!(comprobar_sucesion(&vec![4, 4, 3, 2, 2, 2, 1]));
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
        assert!(Arista::<i32, isize>::sumatorio_pesos(arbol.get_aristas()) == 39);
    }

    #[test]
    fn test_arbol_anchura()
    {
        let g: Grafo<char> = Grafo::from_aristas([Arista::arista_sin_peso('v', 'r'),
                                                        Arista::arista_sin_peso('s', 'r'),
                                                        Arista::arista_sin_peso('w', 's'),
                                                        Arista::arista_sin_peso('w', 'x'),
                                                        Arista::arista_sin_peso('x', 'y'),
                                                        Arista::arista_sin_peso('x', 't'),
                                                        Arista::arista_sin_peso('x', 'u'),
                                                        Arista::arista_sin_peso('t', 'u'),
                                                        Arista::arista_sin_peso('t', 'w'),
                                                        Arista::arista_sin_peso('y', 'u')].to_vec());
        let anchura = arbol_anchura(&g, &'v').expect("El arbol debe existr").into_grafo();
        for arista in anchura.get_aristas()
        {
            if let Arista::Arista(v, w, _) = arista
            {
                println!("ARISTA: {} -> {}", v, w);
            }
            else {
                panic!("No deben haber vertices aislados en el resultado");
            }
        }
        
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
    fn test_dijkstra_1()
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

    #[test]
    fn test_dijkstra_2()
    {
        let g: Grafo<char, usize> = Grafo::from_aristas([Arista::arista('A', 'B', Some(3)),
                                                                Arista::arista('A', 'C', Some(1)),
                                                                Arista::arista('B', 'C', Some(7)),
                                                                Arista::arista('B', 'E', Some(1)),
                                                                Arista::arista('B', 'D', Some(5)),
                                                                Arista::arista('C', 'D', Some(2)),
                                                                Arista::arista('D', 'E', Some(7))].to_vec());

        let min = arbol_camino_minimo(&g, &'C').expect("El arbol debe existir");
        let distancias = min.1;
        assert_eq!(distancias.buscar_vertice(&'A').unwrap().get_valor(), 1);
        assert_eq!(distancias.buscar_vertice(&'B').unwrap().get_valor(), 4);
        assert_eq!(distancias.buscar_vertice(&'D').unwrap().get_valor(), 2);
        assert_eq!(distancias.buscar_vertice(&'E').unwrap().get_valor(), 5);
    }

    #[test]
    fn test_dijkstra_3()
    {
        let dg: Digrafo<i32, usize> = Digrafo::from_aristas([Diarista::arista(1, 2, Some(7)),
                                                                Diarista::arista(1, 4, Some(2)),
                                                                Diarista::arista(2, 4, Some(2)),
                                                                Diarista::arista(4, 2, Some(3)),
                                                                Diarista::arista(2, 3, Some(1)),
                                                                Diarista::arista(4, 3, Some(8))].to_vec());
        let min = arbol_camino_minimo(&dg, &1)
                        .expect("El arbol debe existir").1;
        assert_eq!(min.buscar_vertice(&1).unwrap().get_valor(), 0);
        assert_eq!(min.buscar_vertice(&2).unwrap().get_valor(), 5);
        assert_eq!(min.buscar_vertice(&3).unwrap().get_valor(), 6);
        assert_eq!(min.buscar_vertice(&4).unwrap().get_valor(), 2);
    }

}
