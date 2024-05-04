pub mod grafo
{
    use crate::grafo_rs::{Arista, AristaT, NoPeso, PesoT, VerticeT};

    mod tests;

    pub struct Grafo<Vertice, Peso = NoPeso> 
    where Vertice: VerticeT, Peso: PesoT {
        lista_aristas: Vec<Arista<Vertice, Peso>>,
    }

    impl<Vertice, Peso> Grafo<Vertice, Peso> 
    where Vertice: VerticeT, Peso: PesoT {
        /// 
        /// PRE: Cierto
        /// POST: Grafo vacio
        /// 
        pub fn new() -> Self
        {
            Grafo{
                lista_aristas: vec![],
            }
        }

        ///
        /// PRE: Grafo
        /// POST: Devuelve el numero de vertices
        /// 
        pub fn size(&self) -> usize
        {
            self.get_vertices().len()
        }

        ///
        /// PRE: Vector con Aristas propias
        /// POST: Grafo contruido a partir de dichas aristas, no repitiendo ninguna
        /// 
        pub fn from_aristas(lista: Vec<Arista<Vertice, Peso>>) -> Self
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
        pub fn add_aristas(&mut self, lista: Vec<Arista<Vertice, Peso>>)
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
        pub fn add_vertices(&mut self, lista: Vec<Vertice>)
        {
            let lista_aristas: Vec<Arista<Vertice, Peso>> = lista.into_iter().map(|x| Arista::<Vertice, Peso>::VerticeAislado(x)).collect();
            self.add_aristas(lista_aristas);
        }

        ///
        /// PRE: Grafo
        /// POST: Referencia a la lista de aristas, incluyendo vertices aislados
        /// 
        pub fn get_aristas(&self) -> &Vec<Arista<Vertice, Peso>>
        {
            &self.lista_aristas
        }

        ///
        /// PRE: Grafo
        /// POST: Vector con referencias a los vertices del grafo
        /// 
        pub fn get_vertices(&self) -> Vec<&Vertice>
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
        pub fn remove_arista(&mut self, e: &Arista<Vertice, Peso>)
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

        pub fn remove_vertice(&mut self, v: &Vertice)
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

        ///
        /// PRE: Vertice al que calcular su entorno
        /// POST: Devuelve un vector con referencias a los vertices adyacentes a v. Si no pertenece al grafo, None
        /// 
        pub fn entorno(&self, v: &Vertice) -> Option<Vec<&Vertice>>
        {
            let mut res = vec![];
            let mut encontrado = false;
            for arista in self.lista_aristas.iter()
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
                    if let Arista::VerticeAislado(v0) = arista
                    {
                        encontrado = *v == *v0;
                    }
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
        pub fn aristas_por_vertice(&self, v: &Vertice) -> Vec<&Arista<Vertice, Peso>>
        {
            let aristas_vertice: Vec<&Arista<Vertice, Peso>> = self.lista_aristas.iter().filter(|x| x.arista_contiene_vertice(v))
                                                            .collect();
            aristas_vertice
        }


        ///
        /// PRE: El vertice al que calcular su grado
        /// POST: Valor opcional con el grado. Si el vertice no esta incluido en el grafo, devuelve None
        /// 
        pub fn grado(&self, v: &Vertice) -> Option<usize>
        {
            let mut encontrado = false;
            let mut result: usize = 0;
            for arista in self.lista_aristas.iter()
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
                    if let Arista::VerticeAislado(v1) = arista
                    {
                        encontrado = *v == *v1;
                    }
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
        pub fn sucesion_grados(&self) -> Vec<usize>
        {
            let mut sucesion = vec![];

            for vertice in self.get_vertices()
            {
                sucesion.push(self.grado(&vertice).unwrap());
            }
            // Ordenar de mayor a menor
            sucesion.sort_by(|g1, g2| g2.cmp(g1));
            sucesion.clone()
        }

        ///
        /// PRE: Sucesion de numeros enteros decreciente
        /// POST: Devuelve cierto en caso de ser sucesion grafica, eoc falso
        /// NOTA: Se implementa usando el Teorema de Havel-Hakimi
        /// 
        pub fn comprobar_sucesion(sucesion: &Vec<isize>) -> bool
        {            
            let mut sucesion = sucesion.clone();
            if sucesion.len() == 0 { return false; }
            sucesion.sort_by(|g1, g2| g2.cmp(g1));
            if sucesion[0] >= sucesion.len().try_into().unwrap() { return false; }

            while sucesion[0] > 0
            {
                let first = sucesion[0];
                // Costruimos un nuevo vector
                sucesion = sucesion.iter().enumerate().map(|(i, e)| {
                    if i > 0 && i <= first.try_into().unwrap()
                    { return *e - 1;}
                    *e
                }).collect();
                sucesion.remove(0);
                sucesion.sort_by(|g1, g2| g2.cmp(g1));
            }
            match sucesion.last() {
                Some(e) => match e.cmp(&0) {
                    std::cmp::Ordering::Less => false,
                    std::cmp::Ordering::Equal => true,
                    std::cmp::Ordering::Greater => { panic!("sucesion.last mayor que 0 !!"); }
                },
                None => false
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

    pub struct Arbol<Vertice, Peso>
    where Vertice: VerticeT, Peso: PesoT
    {
        grafo: Grafo<Vertice, Peso>,
        raiz: Vertice
    }

    impl<Vertice, Peso> Arbol<Vertice, Peso> 
    where Vertice: VerticeT, Peso: PesoT
    {        
        pub fn new(grafo: Grafo<Vertice, Peso>, raiz: Vertice) -> Self
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

    impl<Vertice, Peso> Clone for Arbol<Vertice, Peso>
    where Vertice: VerticeT, Peso: PesoT
    {
        fn clone(&self) -> Self {
            Self{
                grafo: self.grafo.clone(),
                raiz: self.raiz.clone()
            }
        }
    }
}
