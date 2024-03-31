pub mod etiquetado
{
    pub struct Etiqueta<'a, Vertice>
    where Vertice: Clone + PartialEq {
        vert: &'a Vertice,
        valor: isize
    }

    impl<'a, Vertice> Clone for Etiqueta<'a, Vertice>
    where Vertice: Clone + PartialEq {
        fn clone(&self) -> Self {
            Self{
                vert: self.vert,
                valor: self.valor
            }
        }
    }

    impl<'a, Vertice> PartialEq for Etiqueta<'a, Vertice>
    where Vertice: Clone + PartialEq {
        fn eq(&self, other: &Self) -> bool {
            self.vert.eq(other.vert) && self.valor == other.valor
        }
    }

    impl<'a, Vertice> Etiqueta<'a, Vertice>
    where Vertice: Clone + PartialEq {
        pub fn get_vertice(&self) -> &Vertice
        {
            self.vert
        }

        pub fn get_valor(&self) -> isize
        {
            self.valor
        }
    }

    pub struct Etiquetado<'a, Vertice>
    where Vertice: Clone + PartialEq {
        datos: Vec<Etiqueta<'a, Vertice>>
    }

    impl<'a, Vertice> Clone for Etiquetado<'a, Vertice> 
    where Vertice: Clone + PartialEq {
        fn clone(&self) -> Self {
            Self{
                datos: self.datos.clone()
            }
        }
    }
}
