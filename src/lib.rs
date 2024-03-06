pub mod grafo_rs
{
    pub enum Arista<Vertice>
    where Vertice: PartialEq, Vertice: Clone {
        Arista(Vertice, Vertice),
        VerticeAislado(Vertice)
    }

    impl<Vertice> Arista<Vertice> 
    where Vertice: PartialEq, Vertice: Clone {
        pub fn arista(v: Vertice, w: Vertice) -> Self
        {
            Self::Arista(v, w)
        }

        pub fn vertice(v: Vertice) -> Self
        {
            Self::VerticeAislado(v)
        }
    }

    impl<Vertice> Clone for Arista<Vertice> 
    where Vertice: PartialEq, Vertice: Clone {
        fn clone(&self) -> Self {
            match &self {
                Arista::Arista(v, w) => Self::Arista(v.clone(), w.clone()),
                Arista::VerticeAislado(v) => Self::VerticeAislado(v.clone())
            }
        }
    }

    impl<Vertice> PartialEq for Arista<Vertice> 
    where Vertice: PartialEq, Vertice: Clone {
        fn eq(&self, other: &Self) -> bool {
            if let Self::Arista(v1, w1) = &self
            {
                if let Self::Arista(v2, w2) = other
                {
                    return (v1 == w1 && v2 == w2) || (v1 == w2 && v2 == w1);
                }
                else 
                {
                    return false;
                }
            }
            if let Self::VerticeAislado(w) = other
            {
                unimplemented!();
            }
            return false;
        }
    }

    pub struct Grafo<Vertice> 
    where Vertice: PartialEq, Vertice: Clone {
        lista_aristas: Vec<Arista<Vertice>>,
    }

    impl<Vertice> Grafo<Vertice> 
    where Vertice: PartialEq, Vertice: Clone {
        pub fn size(&self) -> usize
        {
            unimplemented!();
        }

        pub fn new() -> Self
        {
            Grafo{
                lista_aristas: vec![],
            }
        }

        pub fn from_aristas(lista: &[Arista<Vertice>]) -> Self
        {
            Self{
                lista_aristas: lista.to_vec()
            }
        }
    }
}