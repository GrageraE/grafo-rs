#[cfg(test)]
mod tests
{
    use super::super::Digrafo;
    use crate::grafo_rs::{Diarista, AristaT, NoPeso, GrafoT};

    #[test]
    fn test_eliminacion_aristas_vertices()
    {
        let mut grafo0: Digrafo<i32, NoPeso> = Digrafo::from_aristas([Diarista::arista_sin_peso(1, 2),
                                                                        Diarista::arista_sin_peso(2, 3),
                                                                        Diarista::arista_sin_peso(3, 4),
                                                                        Diarista::VerticeAislado(10),
                                                                        Diarista::arista_sin_peso(3, 2)].to_vec());

        assert_eq!(grafo0.grado(&10), Some(0), "Comprobar que el vertice 10 tiene grado 0");
        grafo0.remove_vertice(&10);
        assert_eq!(grafo0.grado(&10), None, "Comprobar que se haya eliminado");

        let mut grafo1 = grafo0.clone();

        grafo0.remove_arista(&Diarista::arista_sin_peso(3, 2));
        assert!(!grafo0.get_aristas().contains(&Diarista::arista_sin_peso(3, 2)));
        assert!(grafo0.get_aristas().contains(&Diarista::arista_sin_peso(2, 3)), 
                "Comprobamos que se ha eliminado la arista correcta");

        grafo1.remove_vertice(&2);
        assert_eq!(grafo1.grado(&3), Some(1));
        assert_eq!(grafo1.grado(&1), Some(0));
    }

    #[test]
    fn test_grafo_subyacente()
    {
        let digrafo: Digrafo<i32> = Digrafo::from_aristas([Diarista::arista_sin_peso(1, 2),
                                                                Diarista::arista_sin_peso(2, 1),
                                                                Diarista::arista_sin_peso(2,3),
                                                                Diarista::vertice(10),
                                                                Diarista::arista_sin_peso(3, 1)].to_vec());
        
        assert_eq!(digrafo.grado(&3), Some(2), "Comprobamos creacion basica");

        let subyacente = digrafo.grafo_subyacente();
        assert_eq!(subyacente.grado(&1), Some(2), "Comprobacion de grados");
        assert_eq!(subyacente.grado(&2), Some(2), "Comprobacion de grados");
        assert_eq!(subyacente.grado(&10), Some(0), "Vertice aislado");
        assert_eq!(subyacente.grado(&3), Some(2), "Comprobacion final");
    }

    #[test]
    fn test_grados()
    {
        let digrafo: Digrafo<i32> = Digrafo::from_aristas([Diarista::arista_sin_peso(1, 2),
                                                                Diarista::arista_sin_peso(2, 1),
                                                                Diarista::arista_sin_peso(5, 6),
                                                                Diarista::arista_sin_peso(4, 5),
                                                                Diarista::arista_sin_peso(4, 2),
                                                                Diarista::vertice(10)].to_vec());
        
        assert_eq!(digrafo.grado_entrada(&10), Some(0), "Vertice aislado");
        assert_eq!(digrafo.grado_salida(&10), Some(0), "Vertice aislado");

        assert_eq!(digrafo.grado_entrada(&2), Some(2), "Grado de entrada de 2");
        assert_eq!(digrafo.grado_salida(&2), Some(1), "Grado de salida de 2");
        assert_eq!(digrafo.grado_entrada(&4), Some(0), "Grado de entrada de 4");
        assert_eq!(digrafo.grado_salida(&4), Some(2), "Grado de salida de 4");

        assert_eq!(digrafo.grado_entrada(&20), None, "Vertice inexistente");
    }

}
