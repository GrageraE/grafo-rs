pub mod arista
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
    where Vertice: Clone + PartialEq, Peso: Clone + PartialEq {
        Arista(Vertice, Vertice, Option<Peso>),
        VerticeAislado(Vertice)
    }

    impl<Vertice, Peso> Arista<Vertice, Peso> 
    where Vertice: Clone + PartialEq, Peso: Clone + PartialEq {
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
    where Vertice: Clone + PartialEq, Peso: Clone + PartialEq {
        fn clone(&self) -> Self {
            match &self {
                Arista::Arista(v, w, p) => 
                    Self::Arista(v.clone(), w.clone(), p.clone()),
                Arista::VerticeAislado(v) => Self::VerticeAislado(v.clone())
            }
        }
    }

    impl<Vertice, Peso> PartialEq for Arista<Vertice, Peso> 
    where Vertice: Clone + PartialEq, Peso: Clone + PartialEq {
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
}
