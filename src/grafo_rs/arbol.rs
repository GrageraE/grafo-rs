use std::marker::PhantomData;

use crate::grafo_rs::{GrafoT, PesoT, VerticeT};

pub struct Arbol<Graf, Vertice, Peso>
where Graf: GrafoT<Vertice, Peso>, Vertice: VerticeT, Peso: PesoT
{
    grafo: Graf,
    raiz: Vertice,
    p: PhantomData<Peso>
}

impl<Graf, Vertice, Peso> GrafoT<Vertice, Peso> for Arbol<Graf, Vertice, Peso>
where Graf: GrafoT<Vertice, Peso>, Vertice: VerticeT, Peso: PesoT
{
    type Arista = Graf::Arista;

    fn new() -> Self {
        unimplemented!("Arbol no puede ser creado")
    }

    fn from_aristas(_: Vec<Self::Arista>) -> Self {
        unimplemented!("Arbol no puede ser creado")
    }

    fn get_aristas(&self) -> &Vec<Self::Arista> {
        self.grafo.get_aristas()
    }

    fn get_vertices(&self) -> Vec<&Vertice> {
        self.grafo.get_vertices()
    }
}

impl<Graf, Vertice, Peso> Arbol<Graf, Vertice, Peso> 
where Graf: GrafoT<Vertice, Peso>, Vertice: VerticeT, Peso: PesoT
{        
    pub fn from_grafo(grafo: Graf, raiz: Vertice) -> Self
    {
        Self{
            grafo,
            raiz,
            p: PhantomData
        }
    }

    pub fn estructura(&self) -> &Graf
    {
        &self.grafo
    }

    pub fn raiz(&self) -> &Vertice
    {
        &self.raiz
    }

    pub fn into_grafo(self) -> Graf
    {
        self.grafo
    }

}

impl<Graf, Vertice, Peso> Clone for Arbol<Graf, Vertice, Peso>
where Graf: GrafoT<Vertice, Peso> + Clone, Vertice: VerticeT, Peso: PesoT
{
    fn clone(&self) -> Self {
        Self{
            grafo: self.grafo.clone(),
            raiz: self.raiz.clone(),
            p: PhantomData
        }
    }
}
