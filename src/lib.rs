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
        pub fn arista(v: Vertice, w: Vertice, p: Option<Peso>) -> Self
        {
            Self::Arista(v, w, p)
        }

        pub fn arista_sin_peso(v: Vertice, w: Vertice) -> Self
        {
            Self::Arista(v, w, None)
        }

        pub fn vertice(v: Vertice) -> Self
        {
            Self::VerticeAislado(v)
        }

        pub fn peso_por_defecto(&self) -> Option<Arista<Vertice, u8>>
        {
            match &self {
                Self::Arista(v, w, _) => Some(Arista::Arista(v.clone(), w.clone(), Some(1))),
                _ => None
            }
        }

        pub fn arista_contiene_vertice(&self, v0: &Vertice) -> bool
        {
            match &self {
                Self::Arista(v, w, _) => *v == *v0 || *w == *v0,
                _ => false
            }
        }

        pub fn get_vertices(&self) -> Option<(&Vertice, &Vertice)>
        {
            if let Self::Arista(v, w, _) = &self
            {
                return Some((v, w));
            }
            None
        }

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
        /// Devuelve el numero de vertices
        pub fn new() -> Self
        {
            Grafo{
                lista_aristas: vec![],
            }
        }

        pub fn size(&self) -> usize
        {
            self.get_vertices().len()
        }

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

        pub fn add_aristas(&mut self, lista: Vec<Arista<Vertice, Peso>>)
        {
            let mut lista_unica: Vec<Arista<Vertice, Peso>> = vec![];

            for arista in lista.into_iter()
            {
                if !lista_unica.contains(&arista)
                {
                    lista_unica.push(arista);
                }
            }
            self.lista_aristas.append(&mut lista_unica);
        }

        pub fn add_vertices(&mut self, lista: Vec<Vertice>)
        {
            let mut lista_aristas: Vec<Arista<Vertice, Peso>> = lista.into_iter().map(|x| Arista::<Vertice, Peso>::VerticeAislado(x)).collect();
            self.lista_aristas.append(&mut lista_aristas);
        }

        pub fn get_aristas(&self) -> &Vec<Arista<Vertice, Peso>>
        {
            &self.lista_aristas
        }

        pub fn get_vertices(&self) -> Vec<Vertice>
        {
            let mut result: Vec<Vertice> = vec![];
            for arista in self.lista_aristas.iter()
            {
                match arista {
                    Arista::Arista(v, w, _) => {
                        if !result.contains(v)
                        {
                            result.push(v.clone());
                        }
                        if !result.contains(w)
                        {
                            result.push(w.clone());
                        }
                    },
                    Arista::VerticeAislado(v) => {
                        if !result.contains(v)
                        {
                            result.push(v.clone());
                        }
                    }
                }
            }
            result
        }

        pub fn entorno(&self, v: &Vertice) -> Vec<&Vertice>
        {
            let mut res = vec![];
            for arista in self.lista_aristas.iter()
            {
                if arista.arista_contiene_vertice(&v)
                {
                    let (v1, v2) = arista.get_vertices().unwrap();
                    match *v == *v1 {
                        true => { res.push(v2); },
                        false => { res.push(v1); }
                    }
                }
            }
            res
        }

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

        pub fn sucesion_grados(&self) -> Vec<usize>
        {
            let mut sucesion = vec![];

            for vertice in self.get_vertices()
            {
                sucesion.push(self.grado(&vertice).unwrap());
            }

            sucesion.sort_by(|g1, g2| g2.cmp(g1));
            sucesion.clone()
        }
    }
}
