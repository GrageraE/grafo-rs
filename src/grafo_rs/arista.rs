pub mod arista
{
    use crate::grafo_rs::PesoT;
    use crate::grafo_rs::NoPeso;

    use crate::grafo_rs::AristaT;
    use crate::grafo_rs::VerticeT;

    pub enum Arista<Vertice, Peso = NoPeso>
    where Vertice: VerticeT, Peso: PesoT {
        Arista(Vertice, Vertice, Option<Peso>),
        VerticeAislado(Vertice)
    }

    impl<Vertice, Peso> Arista<Vertice, Peso> 
    where Vertice: VerticeT, Peso: PesoT {
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

        ///
        /// PRE: Referencia a vertice
        /// POST: Si es una arista que contiene al vertice, devuelve la otra. None eoc.
        /// 
        pub fn other(&self, v: &Vertice) -> Option<&Vertice>
        {
            if let Arista::Arista(w1, w2, _) = self
            {
                if w1 == v
                {
                    return Some(w2);
                }
                if w2 == v
                {
                    return Some(w1);
                }
            }
            None
        }

        ///
        /// PRE: Referencia a lista de aristas
        /// POST: Referencia a Arista con peso minimo. Si la lista es vacia, None.
        /// NOTA: Requiere que el Peso tenga un orden parcial definido. Puede usarse la funcion peso_por_defecto.
        /// 
        // TODO: Mejorar modelo de ownership de aristas para evitar copias innecesarias
        pub fn min_aristas(aristas: Vec<&Arista<Vertice, Peso>>) -> Option<&Arista<Vertice, Peso>>
        where Peso: PartialOrd
        {
            let mut res = aristas.get(0)?;

            for i in 1..aristas.len()
            {
                if let Some(cmp) = aristas[i].partial_cmp(res)
                {
                    if let std::cmp::Ordering::Less = cmp
                    {
                        res = &aristas[i];
                    }
                }
            }

            Some(res)
        }

        ///
        /// PRE: Vector con referencias a Aristas con Peso = isize
        /// POST: Si todas tienen peso devuelve la suma de los pesos. None eoc
        /// 
        pub fn sumatorio_pesos(aristas: &Vec<Arista<Vertice, isize>>) -> Option<isize>
        {
            aristas.iter().filter(|x| {
                match x {
                    Arista::Arista(_, _, _) => true,
                    _ => false
                }
            })
            .map(|x| x.get_peso())
            .sum()
        }
    }

    impl<Vertice, Peso> Clone for Arista<Vertice, Peso> 
    where Vertice: VerticeT, Peso: PesoT {
        fn clone(&self) -> Self {
            match &self {
                Arista::Arista(v, w, p) => 
                    Self::Arista(v.clone(), w.clone(), p.clone()),
                Arista::VerticeAislado(v) => Self::VerticeAislado(v.clone())
            }
        }
    }

    impl<Vertice, Peso> PartialEq for Arista<Vertice, Peso> 
    where Vertice: VerticeT, Peso: PesoT {
        fn eq(&self, other: &Self) -> bool {
            match &self {
                Self::Arista(v1, w1, p1) => {
                    if let Self::Arista(v2, w2, p2) = other
                    {
                        return (v1 == v2 && w1 == w2 && p1 == p2) || (v1 == w2 && v2 == w1 && p1 == p2);
                    }
                },
                Self::VerticeAislado(v1) => {
                    if let Self::VerticeAislado(v2) = other
                    {
                        return v1 == v2;
                    }
                }
            }
            false
        }
    }

    impl<Vertice, Peso> PartialOrd for Arista<Vertice, Peso>
    where Vertice: VerticeT, Peso: PesoT + PartialOrd
    {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            if let Arista::Arista(_, _, p1) = self
            {
                if let Arista::Arista(_, _, p2) = other
                {
                    return p1.partial_cmp(p2);
                }
            }
            None
        }
    }
}
