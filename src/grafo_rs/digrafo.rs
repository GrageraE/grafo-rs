pub mod digrafo
{
    use crate::grafo_rs::{Arista, AristaT, Diarista, Grafo, GrafoT, NoPeso, PesoT, VerticeT};

    mod tests;

    pub struct Digrafo<Vertice, Peso = NoPeso>
    where Vertice: VerticeT, Peso: PesoT {
        lista_arcos: Vec<Diarista<Vertice, Peso>>
    }

    impl<Vertice, Peso> Digrafo<Vertice, Peso>
    where Vertice: VerticeT, Peso: PesoT
    {
        ///
        /// PRE: true
        /// POST: Grafo subyacente conservando pesos
        /// 
        pub fn grafo_subyacente(&self) -> Grafo<Vertice, Peso>
        {
            let aristas: Vec<Arista<Vertice, Peso>> = self.lista_arcos.iter()
                                        .map(|x| x.clone().into_arista())
                                        .collect();
            Grafo::from_aristas(aristas)
        }

        ///
        /// PRE: Vertice
        /// POST: Grado de entrada del vertice si esta en el digrado. None eoc
        /// 
        pub fn grado_entrada(&self, v: &Vertice) -> Option<usize>
        {
            let mut res = None;
            for arco in self.lista_arcos.iter()
            {
                match arco {
                    Diarista::Diarista(u1, u2, _) => {
                        if u2 == v
                        {
                            res = match res {
                                None => Some(1),
                                Some(d) => Some(d + 1)
                            };
                        }
                        else if let None = res
                        {
                            if u1 == v
                            {
                                res = Some(0);
                            }
                        }
                    },
                    Diarista::VerticeAislado(u) => {
                        if u == v
                        {
                            if let None = res
                            {
                                res = Some(0);
                            }
                        }
                    }
                };
            }
            res
        }

        ///
        /// PRE: Vertice
        /// POST: Grado de salida del vertice si esta en el digrafo. None eoc
        /// 
        pub fn grado_salida(&self, v: &Vertice) -> Option<usize>
        {
            let mut res = None;

            for arco in self.lista_arcos.iter()
            {
                match arco {
                    Diarista::Diarista(u1, u2, _) => {
                        if u1 == v
                        {
                            res = match res {
                                None => Some(1),
                                Some(d) => Some(d + 1)
                            };
                        }
                        else if let None = res
                        {
                            if u2 == v
                            {
                                res = Some(0);
                            }
                        }
                    },
                    Diarista::VerticeAislado(u) => {
                        if u == v
                        {
                            if let None = res
                            {
                                res = Some(0);
                            }
                        }
                    }
                };
            }

            res
        }
    }

    impl<Vertice, Peso> GrafoT<Vertice, Peso> for Digrafo<Vertice, Peso> 
    where Vertice: VerticeT, Peso: PesoT {
        type Arista = Diarista<Vertice, Peso>;

        fn new() -> Self {
            Self{
                lista_arcos: vec![]
            }
        }

        fn from_aristas(lista: Vec<Diarista<Vertice, Peso>>) -> Self 
        {
            let mut lista_arcos = Vec::with_capacity(lista.len());

            for diarista in lista.into_iter()
            {
                if !lista_arcos.contains(&diarista)
                {
                    lista_arcos.push(diarista);
                }
            }
            Self{
                lista_arcos
            }
        }

        fn add_aristas(&mut self, lista: Vec<Diarista<Vertice, Peso>>) 
        {
            for diarista in lista.into_iter()
            {
                if !self.lista_arcos.contains(&diarista)
                {
                    self.lista_arcos.push(diarista);
                }
            }
        }

        fn add_vertices(&mut self, lista: Vec<Vertice>) 
        {
            self.add_aristas(lista.into_iter().map(|x| Diarista::vertice(x)).collect());
        }

        fn get_aristas(&self) -> &Vec<Diarista<Vertice, Peso>> 
        {
            &self.lista_arcos    
        }

        fn get_vertices(&self) -> Vec<&Vertice> 
        {
            let mut lista_vertices: Vec<&Vertice> = vec![];

            for diarista in self.get_aristas().iter()
            {
                match diarista {
                    Diarista::Diarista(v, w, _) => {
                        if !lista_vertices.contains(&v) 
                        {
                            lista_vertices.push(v);
                        }
                        if !lista_vertices.contains(&w)
                        {
                            lista_vertices.push(w);
                        }
                    },
                    Diarista::VerticeAislado(v) => {
                        if !lista_vertices.contains(&v)
                        {
                            lista_vertices.push(v);
                        }
                    }
                }
            }
            lista_vertices    
        }

        fn remove_arista(&mut self, arista: &Diarista<Vertice, Peso>) 
        {
            if let Some(index) = self.get_aristas().iter().position(|x| x == arista)
            {
                let e = self.lista_arcos.remove(index);
                if let Diarista::Diarista(v, w, _) = e
                {
                    self.add_vertices(vec![v, w]);
                }
            }
        }

        fn remove_vertice(&mut self, v: &Vertice) 
        {
            let arista_indexes: Vec<usize> = self.get_aristas().iter().enumerate()
                                                .filter(|x| 
                                                        x.1.arista_contiene_vertice(v) || x.1.es_vetice_aislado(v))
                                                .map(|x| x.0)
                                                .collect();

            let mut i: usize = 0;
            for index in arista_indexes.into_iter()
            {
                let e = self.lista_arcos.remove(index - i);
                if !e.es_vetice_aislado(v)
                {
                    self.lista_arcos.push(Diarista::vertice(e.other(v).unwrap().clone()));
                }
                i += 1;
            }
        }

        ///
        /// PRE: Falso
        /// POST: Nada
        /// NOTA: No se define entorno para Digrafo
        /// 
        fn entorno<'a>(&'a self, _: &Vertice) -> Option<Vec<&Vertice>>
        where Diarista<Vertice, Peso>: 'a 
        {
            unimplemented!("Digrafo no tiene entorno definido")
        }
    }

    impl<Vertice, Peso> Clone for Digrafo<Vertice, Peso>
    where Vertice: VerticeT, Peso: PesoT
    {
        fn clone(&self) -> Self {
            Self{
                lista_arcos: self.lista_arcos.clone()
            }
        }
    }
}
