use crate::grafo_rs::{Arista, AristaT, GrafoT, PesoT, VerticeT};

use super::Grafo;

#[cfg(test)]
mod tests;

pub struct Bipartido<Vertice, Peso>
where Vertice: VerticeT, Peso: PesoT
{
    /*
     * Almacenaremos las aristas de la forma (x,y)
     * Garantizaremos que no tiene vertices aislados
     */    
    lista_aristas: Vec<Arista<Vertice, Peso>>,
}

impl<Vertice, Peso> Bipartido<Vertice, Peso>
where Vertice: VerticeT, Peso: PesoT
{
    ///
    /// PRE: Referencia a Grafo
    /// 
    /// POST: Si el grafo dado es bipartido, genera un Bipartido a partir de el. None eoc
    /// 
    pub fn from_grafo(grafo: &Grafo<Vertice, Peso>) -> Option<Self>
    {
        let mut vertices_x: Vec<&Vertice> = vec![];
        let mut vertices_y: Vec<&Vertice> = vec![];

        let mut lista_aristas: Vec<Arista<Vertice, Peso>> = vec![];
        for arista in grafo.get_aristas().iter()
        {            
            let (v1, v2) = match arista.get_vertices() {
                Some((v1, v2)) => (v1, v2),
                None => {continue;}
            };

            if vertices_x.contains(&v1) {
                if vertices_x.contains(&v2) {
                    return None;
                }
                if !vertices_y.contains(&v2) {
                    vertices_y.push(v2);
                }
                lista_aristas.push(Arista::Arista(v1.clone(), v2.clone(), arista.get_peso().cloned()));
            }
            else if vertices_y.contains(&v1) {
                if vertices_y.contains(&v2) {
                    return None;
                }
                if !vertices_x.contains(&v2) {
                    vertices_x.push(v2);
                }
                lista_aristas.push(Arista::Arista(v2.clone(), v1.clone(), arista.get_peso().cloned()));
            }
            else if vertices_x.contains(&v2) {
                vertices_y.push(v1);
                lista_aristas.push(Arista::Arista(v2.clone(), v1.clone(), arista.get_peso().cloned()));
            }
            else if vertices_y.contains(&v2) {
                vertices_x.push(v1);
                lista_aristas.push(Arista::Arista(v1.clone(), v2.clone(), arista.get_peso().cloned()));
            }
            else {
                vertices_x.push(v1);
                vertices_y.push(v2);
                lista_aristas.push(Arista::Arista(v1.clone(), v2.clone(), arista.get_peso().cloned()));    
            }
        }
        Some(Self {
            lista_aristas
        })
    }

    ///
    /// POST: Consume el Bipartido y devuelve su [`Grafo`] subyacente
    /// 
    pub fn into_grafo(self) -> Grafo<Vertice, Peso>
    {
        Grafo::from_aristas(self.lista_aristas)
    }

    ///
    /// POST: Devuelve referencias a los vertices de la primera particion
    /// 
    pub fn get_vertices_x(&self) -> Vec<&Vertice>
    {
        let mut vertices_x = vec![];
        for arista in self.lista_aristas.iter()
        {
            let v = arista.get_vertices().unwrap().0;
            if !vertices_x.contains(&v) {
                vertices_x.push(v);
            }
        }
        vertices_x
    }

    ///
    /// POST: Devuelve las referencias a los vertices de la segunda particion
    /// 
    pub fn get_vertices_y(&self) -> Vec<&Vertice>
    {
        let mut vertices_y = vec![];
        for arista in self.lista_aristas.iter()
        {
            let v = arista.get_vertices().unwrap().1;
            if !vertices_y.contains(&v) {
                vertices_y.push(v);
            }
        }
        vertices_y
    }
}

impl<Vertice, Peso> GrafoT<Vertice, Peso> for Bipartido<Vertice, Peso>
where Vertice: VerticeT, Peso: PesoT
{
    type Arista = Arista<Vertice, Peso>;

    fn new() -> Self {
        unimplemented!("No se permite crear grafos bipartidos directamente")
    }

    fn from_aristas(_: Vec<Self::Arista>) -> Self {
        unimplemented!("No se permite crear grafos bipartidos directamente")
    }

    fn get_aristas(&self) -> &Vec<Self::Arista> {
        &self.lista_aristas
    }

    fn get_vertices(&self) -> Vec<&Vertice> {
        let mut result: Vec<&Vertice> = vec![];
        for arista in self.lista_aristas.iter()
        {
            let (x, y) = arista.get_vertices().unwrap();
            if !result.contains(&x)
            {
                result.push(x);
            }
            if !result.contains(&y)
            {
                result.push(y);
            }
        }
        result
    }

    fn entorno<'b>(&'b self, v: &Vertice) -> Option<Vec<&Vertice>>
    where Self::Arista: 'b
    {
        let res: Vec<&Vertice> = self.lista_aristas.iter().filter_map(|x| {
            match x.arista_contiene_vertice(v) {
                true => Some(x.other(v).unwrap()),
                _ => None
            }
        }).collect();
        if res.is_empty() {
            // No existen vertices aislados en Bipartido
            return None;
        }
        Some(res)
    }
}

impl<Vertice, Peso> Clone for Bipartido<Vertice, Peso>
where Vertice: VerticeT, Peso: PesoT
{
    fn clone(&self) -> Self {
        Self {
            lista_aristas: self.lista_aristas.clone(),
        }
    }
}
