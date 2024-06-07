use crate::grafo_rs::{Arista, AristaT, GrafoT, PesoT, VerticeT};

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
    pub fn get_vertices_x(&self) -> Vec<&Vertice>
    {
        self.lista_aristas.iter().map(|x| x.get_vertices().unwrap().0).collect()
    }

    pub fn get_vertices_y(&self) -> Vec<&Vertice>
    {
        self.lista_aristas.iter().map(|x| x.get_vertices().unwrap().1).collect()
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
