pub mod grafo_t
{
    use crate::grafo_rs::{AristaT, VerticeT, PesoT};

    pub trait GrafoT<Vertice, Peso, Arista>
    where Vertice: VerticeT, Peso: PesoT, Arista: AristaT<Vertice, Peso>
    {
        fn new() -> Self;

        ///
        /// PRE: Grafo
        /// POST: Devuelve el numero de vertices
        /// 
        fn size(&self) -> usize
        {
            self.get_vertices().len()
        }

        fn from_aristas(lista: Vec<Arista>) -> Self;

        fn add_aristas(&mut self, lista: Vec<Arista>);

        fn add_vertices(&mut self, lista: Vec<Vertice>);

        fn get_aristas(&self) -> &Vec<Arista>;

        fn get_vertices(&self) -> Vec<&Vertice>;

        fn remove_arista(&mut self, e: &Arista);

        fn remove_vertice(&mut self, v: &Vertice);

        ///
        /// PRE: Vertice al que calcular su entorno
        /// POST: Devuelve un vector con referencias a los vertices adyacentes a v. Si no pertenece al grafo, None
        /// 
        fn entorno<'a>(&'a self, v: &Vertice) -> Option<Vec<&Vertice>>
        where Arista: 'a
        {
            let mut res = vec![];
            let mut encontrado = false;
            for arista in self.get_aristas().iter()
            {
                if arista.arista_contiene_vertice(&v)
                {
                    encontrado = true;
                    let (v1, v2) = arista.get_vertices().unwrap();
                    match *v == *v1 {
                        true => { res.push(v2); },
                        false => { res.push(v1); }
                    }
                }
                else if !encontrado
                {
                    encontrado = arista.es_vetice_aislado(v);
                }
            }
            match encontrado {
                true => Some(res),
                _ => None
            }

        }

        ///
        /// PRE: El vertice tiene que pertenecer al grafo
        /// POST: Vector con referencias a las aristas
        /// 
        fn aristas_por_vertice(&self, v: &Vertice) -> Vec<&Arista>
        {
            self.get_aristas().iter().filter(|x| x.arista_contiene_vertice(v)).collect()
        }

        ///
        /// PRE: El vertice al que calcular su grado
        /// POST: Valor opcional con el grado. Si el vertice no esta incluido en el grafo, devuelve None
        /// 
        fn grado(&self, v: &Vertice) -> Option<usize>
        {
            let mut encontrado = false;
            let mut result: usize = 0;
            for arista in self.get_aristas().iter()
            {
                if arista.arista_contiene_vertice(&v)
                {
                    encontrado = true;
                    result += 1;
                    // Vemos si es un ciclo
                    let (v1, v2) = arista.get_vertices().unwrap();
                    if *v1 == *v2
                    {
                        result += 1;
                    }
                }
                else if !encontrado
                {
                   encontrado = arista.es_vetice_aislado(v);
                }
            }
            match encontrado {
                true => Some(result),
                _ => None
            }
        }

        ///
        /// PRE: Grafo
        /// POST: Vector con la sucesion decreciente de grados
        /// 
        fn sucesion_grados(&self) -> Vec<usize>
        {
            let mut sucesion = vec![];

            for vertice in self.get_vertices()
            {
                sucesion.push(self.grado(&vertice).unwrap());
            }
            // Ordenar de mayor a menor
            sucesion.sort_by(|g1, g2| g2.cmp(g1));
            sucesion
        }
    }
}
