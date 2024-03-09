pub mod grafo_rs
{
    pub struct NoPeso;

    impl PartialEq for NoPeso {
        fn eq(&self, _: &Self) -> bool {
            true
        }
    }

    impl Clone for NoPeso {
        fn clone(&self) -> Self {
            Self
        }
    }

    pub enum Arista<Vertice, Peso = NoPeso>
    where Vertice: PartialEq, Vertice: Clone, Peso: PartialEq, Peso: Clone {
        Arista(Vertice, Vertice, Option<Peso>),
        VerticeAislado(Vertice)
    }

    impl<Vertice, Peso> Arista<Vertice, Peso> 
    where Vertice: PartialEq, Vertice: Clone, Peso: PartialEq, Peso: Clone {
        ///
        /// PRE: Dos vertices y un peso opcional
        /// POST: Arista formada por dichos vertices y el peso opcional
        ///
        pub fn arista(v: Vertice, w: Vertice, p: Option<Peso>) -> Self
        {
            Self::Arista(v, w, p)
        }

        ///
        /// PRE: Dos vertices
        /// POST: Arista formada por dichos vertices, sin peso
        /// 
        pub fn arista_sin_peso(v: Vertice, w: Vertice) -> Self
        {
            Self::Arista(v, w, None)
        }

        ///
        /// PRE: Vertice
        /// POST: Vertice aislado
        /// 
        pub fn vertice(v: Vertice) -> Self
        {
            Self::VerticeAislado(v)
        }

        ///
        /// PRE: Arista
        /// POST: Una arista formada por los vertices de la anterior con peso 1, si es arista. None eoc
        /// 
        pub fn peso_por_defecto(&self) -> Option<Arista<Vertice, u8>>
        {
            match &self {
                Self::Arista(v, w, _) => Some(Arista::Arista(v.clone(), w.clone(), Some(1))),
                _ => None
            }
        }

        ///
        /// PRE: Vertice
        /// POST: Cierto si es una arista y contiene a v. Falso si no contiene a v o es vertice aislado
        /// 
        pub fn arista_contiene_vertice(&self, v0: &Vertice) -> bool
        {
            match &self {
                Self::Arista(v, w, _) => *v == *v0 || *w == *v0,
                _ => false
            }
        }

        ///
        /// PRE: Arista
        /// POST: Tupla con referencias a los vetices si es arista. None eoc
        /// 
        pub fn get_vertices(&self) -> Option<(&Vertice, &Vertice)>
        {
            if let Self::Arista(v, w, _) = &self
            {
                return Some((v, w));
            }
            None
        }

        ///
        /// PRE: Arista y Vertice
        /// POST: Cieto si es vertice aislado y contiene a v. Falso eoc
        /// 
        pub fn es_vetice_aislado(&self, v: &Vertice) -> bool
        {
            if let Self::VerticeAislado(v0) = &self
            {
                return *v0 == *v;
            }
            false
        }

        ///
        /// PRE: Arista
        /// POST: Referencia a peso si lo contiene. None eoc
        /// 
        pub fn get_peso(&self) -> Option<&Peso>
        {
            if let Self::Arista(_, _, p) = &self
            {
                return p.as_ref();
            }
            None
        }
    }

    impl<Vertice, Peso> Clone for Arista<Vertice, Peso> 
    where Vertice: PartialEq, Vertice: Clone, Peso: PartialEq, Peso: Clone {
        fn clone(&self) -> Self {
            match &self {
                Arista::Arista(v, w, p) => 
                    Self::Arista(v.clone(), w.clone(), p.clone()),
                Arista::VerticeAislado(v) => Self::VerticeAislado(v.clone())
            }
        }
    }

    impl<Vertice, Peso> PartialEq for Arista<Vertice, Peso> 
    where Vertice: PartialEq, Vertice: Clone, Peso: PartialEq, Peso: Clone {
        fn eq(&self, other: &Self) -> bool {
            match &self {
                Self::Arista(v1, w1, p1) => {
                    if let Self::Arista(v2, w2, p2) = other
                    {
                        return (v1 == v2 && w1 == w2 && p1 == p2) || (v1 == w2 && v2 == w1 && p1 == p2);
                    }
                    else 
                    {
                        return false;
                    }
                },
                Self::VerticeAislado(v1) => {
                    if let Self::VerticeAislado(v2) = other
                    {
                        return v1 == v2;
                    }
                    else 
                    {
                        return false;
                    }
                }
            }
        }
    }

    pub struct Grafo<Vertice, Peso = NoPeso> 
    where Vertice: PartialEq, Vertice: Clone, Peso: PartialEq, Peso: Clone {
        lista_aristas: Vec<Arista<Vertice, Peso>>,
    }

    impl<Vertice, Peso> Grafo<Vertice, Peso> 
    where Vertice: PartialEq, Vertice: Clone, Peso: PartialEq, Peso: Clone {
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
                self.lista_aristas.remove(index);
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
            for index in lista_aristas_index
            {
                self.lista_aristas.remove(index);
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
    where Vertice: PartialEq, Vertice: Clone, Peso: PartialEq, Peso: Clone {
        fn clone(&self) -> Self 
        {
            Self {
                lista_aristas: self.lista_aristas.clone()
            }
        }
    }
}
