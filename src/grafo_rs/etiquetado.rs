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

    impl<Vertice> PartialOrd for Etiqueta<Vertice>
    where Vertice: Clone + PartialEq
    {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.valor.cmp(&other.valor))
        }
    }

    impl<Vertice> Etiqueta<Vertice>
    where Vertice: Clone + PartialEq {
        ///
        /// PRE: Vertice propio y valor de la etiqueta
        /// POST: Objeto Etiqueta
        /// 
        pub fn new(vert: Vertice, valor: isize) -> Self
        {
            Self{
                vert,
                valor
            }
        }

        ///
        /// PRE: Nuevo valor
        /// POST: Etiqueta con valor cambiado
        /// 
        pub fn set_valor(&mut self, valor: isize)
        {
            self.valor = valor;
        }

        ///
        /// PRE: Cierto
        /// POST: Referencia al vertice etiquetado
        /// 
        pub fn get_vertice(&self) -> &Vertice
        {
            &self.vert
        }

        ///
        /// PRE: Cierto
        /// POST: Etiqueta asignada
        /// 
        pub fn get_valor(&self) -> isize
        {
            self.valor
        }
    }

    pub struct Etiquetado<Vertice>
    where Vertice: Clone + PartialEq {
        nombre: Option<String>,
        datos: Vec<Etiqueta<Vertice>>
    }

    impl<Vertice> Clone for Etiquetado<Vertice> 
    where Vertice: Clone + PartialEq {
        fn clone(&self) -> Self {
            Self{
                nombre: self.nombre.clone(),
                datos: self.datos.clone()
            }
        }
    }

    impl<Vertice> Etiquetado<Vertice>
    where Vertice: Clone + PartialEq
    {
        ///
        /// PRE: Opcion a un nombre. Dicho nombre puede ser clonado
        /// POST: Objeto Etiquetado vacio
        /// 
        pub fn new(nombre: Option<&str>) -> Self
        {
            let nombre = match nombre {
                Some(s) => Some(s.to_owned()),
                None => None
            };
            Self{
                nombre,
                datos: Vec::new()
            }
        }

        ///
        /// PRE: Opcion a un nombre que puede ser clonado y una lista de Etiquetas.
        /// POST: Objeto Etiquetado con la lista
        /// 
        pub fn from_vec(nombre: Option<&str>, list: Vec<Etiqueta<Vertice>>) -> Self
        {
            let nombre = match nombre {
                Some(s) => Some(s.to_owned()),
                None => None
            };
            Self{
                nombre,
                datos: list
            }
        }

        ///
        /// PRE: Objeto mutable y etiqueta
        /// POST: Etiquetado actualizado
        /// 
        pub fn add_etiqueta(&mut self, etiqueta: Etiqueta<Vertice>)
        {
            self.datos.push(etiqueta);
        }

        ///
        /// PRE: Objeto mutable, vertice propio y valor de etiqueta
        /// POST: Etiquetado actualizado
        /// 
        pub fn add_vertice(&mut self, v: Vertice, valor: isize)
        {
            self.datos.push(Etiqueta::new(v, valor));
        }

        ///
        /// PRE: Cierto
        /// POST: Referencia a etiqueta maxima. None si el etiquetado esta vacio
        /// 
        pub fn max(&self) -> Option<&Etiqueta<Vertice>>
        {
            if self.datos.is_empty()
            {
                return None;
            }

            let mut max = self.datos.get(0).unwrap();
            for etiqueta in self.datos[1..].iter()
            {
                if etiqueta > max
                {
                    max = etiqueta;
                }
            }
            Some(max)
        }

        ///
        /// PRE: Cierto
        /// POST: Referencia a etiqueta minima. None si el etiquetado esta vacio
        /// 
        pub fn min(&self) -> Option<&Etiqueta<Vertice>>
        {
            if self.datos.is_empty()
            {
                return None;
            }

            let mut min = self.datos.get(0).unwrap();
            for etiqueta in self.datos[1..].iter()
            {
                if etiqueta < min
                {
                    min = etiqueta;
                }
            }
            Some(min)
        }

        ///
        /// PRE: Vertice
        /// POST: Referencia a etiqueta con el vertice dado, si existe en el etiquetado. None si no se encuentra
        /// 
        pub fn buscar_vertice(&self, v: &Vertice) -> Option<&Etiqueta<Vertice>>
        {
            let etiqueta_vertice: Vec<&Etiqueta<Vertice>> = self.datos.iter().filter(|x| x.get_vertice() == v)
                                                .collect();
            if etiqueta_vertice.len() == 1
            {
                return Some(etiqueta_vertice[0]);
            }
            None
        }

        pub fn buscar_vertice_mut(&mut self, v: &Vertice) -> Option<&mut Etiqueta<Vertice>>
        {
            let etiqueta_vertice_index: Vec<usize> = self.datos.iter().enumerate()
                                                    .filter(|x| x.1.get_vertice() == v)
                                                    .map(|x| x.0)
                                                    .collect();
            
            if etiqueta_vertice_index.len() == 1
            {
                return Some(self.datos.get_mut(etiqueta_vertice_index[0]).unwrap());
            }
            None
        }

        ///
        /// PRE: Cierto
        /// POST: Referencia constante a vector con las etiquetas
        /// 
        pub fn get_datos(&self) -> &Vec<Etiqueta<Vertice>>
        {
            &self.datos
        }

        ///
        /// PRE: Cierto
        /// POST: Opcion a referencia al nombre.
        /// 
        pub fn get_nombre(&self) -> Option<&str>
        {
            self.nombre.as_deref()
        }

        ///
        /// PRE: Cierto
        /// POST: El objeto Etiquetado se destruye y se devuelve su vector con propiedad.
        /// 
        pub fn into_vec(self) -> Vec<Etiqueta<Vertice>>
        {
            self.datos
        }
    }
}
