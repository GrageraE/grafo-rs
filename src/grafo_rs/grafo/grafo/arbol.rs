pub mod arbol
{
    use super::super::Grafo;

    use crate::grafo_rs::{Arista, GrafoT, PesoT, VerticeT};

    pub struct Arbol<Graf, Vertice, Peso>
    where Graf: GrafoT<Vertice, Peso>, Vertice: VerticeT, Peso: PesoT
    {
        grafo: Graf,
        raiz: Vertice
    }

    impl<Graf, Vertice, Peso> GrafoT<Vertice, Peso> for Arbol<Graf, Vertice, Peso>
    where Graf: GrafoT<Vertice, Peso>, Vertice: VerticeT, Peso: PesoT
    {
        type Arista = Arista<Vertice, Peso>;

        fn new() -> Self {
            unimplemented!("Arbol no puede ser creado")
        }

        fn from_aristas(_: Vec<Arista<Vertice, Peso>>) -> Self {
            unimplemented!("Arbol no puede ser creado")
        }

        fn get_aristas(&self) -> &Vec<Arista<Vertice, Peso>> {
            self.grafo.get_aristas()
        }

        fn get_vertices(&self) -> Vec<&Vertice> {
            self.grafo.get_vertices()
        }
    }

    impl<Graf, Vertice, Peso> Arbol<Graf, Vertice, Peso> 
    where Graf: GrafoT<Vertice, Peso>, Vertice: VerticeT, Peso: PesoT
    {        
        pub fn from_grafo(grafo: Grafo<Vertice, Peso>, raiz: Vertice) -> Self
        {
            Self{
                grafo,
                raiz
            }
        }

        pub fn estructura(&self) -> &Grafo<Vertice, Peso>
        {
            &self.grafo
        }

        pub fn raiz(&self) -> &Vertice
        {
            &self.raiz
        }

        pub fn into_grafo(self) -> Grafo<Vertice, Peso>
        {
            self.grafo
        }

    }

    impl<Graf, Vertice, Peso> Clone for Arbol<Graf, Vertice, Peso>
    where Graf: GrafoT<Vertice, Peso>, Vertice: VerticeT, Peso: PesoT
    {
        fn clone(&self) -> Self {
            Self{
                grafo: self.grafo.clone(),
                raiz: self.raiz.clone()
            }
        }
    }
}
