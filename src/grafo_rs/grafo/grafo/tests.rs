#[cfg(test)]
mod tests
{
    use super::super::{Arista, AristaT, Grafo, GrafoT, NoPeso};
    
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
        let grafo0: Grafo<i32, NoPeso> = Grafo::from_aristas([Arista::arista_sin_peso(1, 2),
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
}
