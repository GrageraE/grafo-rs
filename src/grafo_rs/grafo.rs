use crate::grafo_rs::{Arista, AristaT, 
                    NoPeso, PesoT, 
                    VerticeT, GrafoT};

#[cfg(test)]
mod tests;

pub mod arbol;
pub use arbol::Arbol;

pub struct Grafo<Vertice, Peso = NoPeso> 
where Vertice: VerticeT, Peso: PesoT {
    lista_aristas: Vec<Arista<Vertice, Peso>>,
}

impl<Vertice, Peso> GrafoT<Vertice, Peso> for Grafo<Vertice, Peso>
where Vertice: VerticeT, Peso: PesoT {
    type Arista = Arista<Vertice, Peso>;

    ///
    /// PRE: true
    /// POST: Grafo vacio
    /// 
    fn new() -> Self 
    {
        Self{
            lista_aristas: vec![]
        }    
    }

    ///
    /// PRE: Vector con Aristas propias
    /// POST: Grafo contruido a partir de dichas aristas, no repitiendo ninguna
    /// 
    fn from_aristas(lista: Vec<Arista<Vertice, Peso>>) -> Self
    {
        let mut lista_aristas: Vec<Arista<Vertice, Peso>> = vec![];

        for arista in lista.into_iter()
        {
            if !lista_aristas.contains(&arista)
            {
                lista_aristas.push(arista);
            }
        }
        Self{
            lista_aristas
        }
    }

    ///
    /// PRE: Grafo modificable y Vector con las Aristas propias
    /// POST: Se modifca el grafo añadiendo las aristas necesarias, no repitiendo ninguna 
    /// 
    fn add_aristas(&mut self, lista: Vec<Arista<Vertice, Peso>>)
    {
        self.lista_aristas.reserve(lista.len());
        for arista in lista.into_iter()
        {
            if !self.lista_aristas.contains(&arista)
            {
                self.lista_aristas.push(arista);
            }
        }
    }

    ///
    /// PRE: Grafo modificable y Vector con Vertices propios
    /// POST. El grafo se modifica y añade los vertices proporcionados
    /// 
    fn add_vertices(&mut self, lista: Vec<Vertice>)
    {
        let lista_aristas: Vec<Arista<Vertice, Peso>> = lista.into_iter().map(|x| Arista::<Vertice, Peso>::VerticeAislado(x)).collect();
        self.add_aristas(lista_aristas);
    }

    ///
    /// PRE: Grafo
    /// POST: Referencia a la lista de aristas, incluyendo vertices aislados
    /// 
    fn get_aristas(&self) -> &Vec<Arista<Vertice, Peso>>
    {
        &self.lista_aristas
    }

    ///
    /// PRE: Grafo
    /// POST: Vector con referencias a los vertices del grafo
    /// 
    fn get_vertices(&self) -> Vec<&Vertice>
    {
        let mut result: Vec<&Vertice> = vec![];
        for arista in self.lista_aristas.iter()
        {
            match arista {
                Arista::Arista(v, w, _) => {
                    if !result.contains(&v)
                    {
                        result.push(v);
                    }
                    if !result.contains(&w)
                    {
                        result.push(w);
                    }
                },
                Arista::VerticeAislado(v) => {
                    if !result.contains(&v)
                    {
                        result.push(v);
                    }
                }
            }
        }
        result
    }

    ///
    /// PRE: Arista que se va a eliminar
    /// POST: Grafo con la arista eliminada
    /// 
    fn remove_arista(&mut self, e: &Arista<Vertice, Peso>)
    {
        // Tenemos en cuenta que la arista tiene una unica ocurrencia en la lista,
        // ya que asi se implementa en los metodos de adicion
        if let Some(index) = self.lista_aristas.iter().position(|x| *x == *e)
        {
            let e = self.lista_aristas.remove(index);
            if let Arista::Arista(v0, w0, _) = e
            {
                self.add_vertices(vec![v0, w0]);
            }
        }
    }

    fn remove_vertice(&mut self, v: &Vertice)
    {
        let lista_aristas_index: Vec<usize> = self.lista_aristas.iter().enumerate()
                                                                .filter(|(_, e)| {
                                                                    e.arista_contiene_vertice(v) || e.es_vetice_aislado(v)
                                                                })
                                                                .map(|(i, _)| i)
                                                                .collect();
        let mut i: usize = 0;
        for index in lista_aristas_index
        {
            let e = self.lista_aristas.remove(index - i);
            if let Arista::Arista(v0, w0, _) = e
            {
                if *v == v0 
                {
                    self.lista_aristas.push(Arista::VerticeAislado(w0));
                }
                else 
                {
                    self.lista_aristas.push(Arista::VerticeAislado(v0));    
                }
            }
            i += 1;
        }
    }
}

impl<Vertice, Peso> Clone for Grafo<Vertice, Peso>
where Vertice: VerticeT, Peso: PesoT {
    fn clone(&self) -> Self 
    {
        Self {
            lista_aristas: self.lista_aristas.clone()
        }
    }
}
