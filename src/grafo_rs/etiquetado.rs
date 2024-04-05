pub mod etiquetado
{
    pub struct Etiqueta<Vertice>
    where Vertice: Clone + PartialEq {
        vert: Vertice,
        valor: isize
    }

    impl<Vertice> Clone for Etiqueta<Vertice>
    where Vertice: Clone + PartialEq {
        fn clone(&self) -> Self {
            Self{
                vert: self.vert.clone(),
                valor: self.valor
            }
        }
    }

    impl<Vertice> PartialEq for Etiqueta<Vertice>
    where Vertice: Clone + PartialEq {
        fn eq(&self, other: &Self) -> bool {
            self.vert.eq(&other.vert) && self.valor == other.valor
        }
    }

    impl<Vertice> Etiqueta<Vertice>
    where Vertice: Clone + PartialEq {
        pub fn get_vertice(&self) -> &Vertice
        {
            &self.vert
        }

        pub fn get_valor(&self) -> isize
        {
            self.valor
        }
    }

    pub struct Etiquetado<Vertice>
    where Vertice: Clone + PartialEq {
        datos: Vec<Etiqueta<Vertice>>
    }

    impl<Vertice> Clone for Etiquetado<Vertice> 
    where Vertice: Clone + PartialEq {
        fn clone(&self) -> Self {
            Self{
                datos: self.datos.clone()
            }
        }
    }
}
