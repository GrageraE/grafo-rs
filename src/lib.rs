pub mod grafo_rs
{
    pub mod arista;
    pub use arista::arista::NoPeso;
    pub use arista::arista::Arista;

    pub use arista::arista::peso::peso::PesoT;

    pub mod etiquetado;
    pub use etiquetado::etiquetado::Etiqueta;
    pub use etiquetado::etiquetado::Etiquetado;

    pub struct Grafo<Vertice, Peso = NoPeso> 
    where Vertice: Clone + PartialEq, Peso: PesoT {
        lista_aristas: Vec<Arista<Vertice, Peso>>,
    }

    impl<Vertice, Peso> Grafo<Vertice, Peso> 
    where Vertice: Clone + PartialEq, Peso: PesoT {
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

        ///
        /// PRE: Cierto
        /// POST: Arbol generador de peso minimo
        /// NOTA: Implementacion del algoritmo de Prim. Requere que el Peso tenga un orden parcial definido
        /// 
        pub fn arbol_peso_minimo(&self) -> Option<Arbol<Vertice, Peso>>
        where Peso: Ord
        {
            let mut arbol = Self::new();
            // Seleccionamos un vertice aleatorio
            let vertice_inicial;
            if let Some(e) = self.lista_aristas.get(0)
            {
                vertice_inicial = match e {
                    Arista::Arista(v, _, _) => v,
                    Arista::VerticeAislado(v) => v
                }
            }
            else 
            {
                return None;
            }

            // Definimos un vector de aristas frontera y de vertices visitados
            let mut aristas_frontera: Vec<&Arista<Vertice, Peso>> = self.lista_aristas.iter()
                                .filter(|x| x.arista_contiene_vertice(vertice_inicial))
                                .collect();

            let mut vertices_visitados = Vec::<&Vertice>::new();
            let mut vertice_visitado = vertice_inicial;

            while !aristas_frontera.is_empty()
            {
                // Añadimos la arista frontera con menor peso
                let arista_minima = Arista::min_aristas(aristas_frontera.clone()).unwrap();
                arbol.add_aristas(vec![arista_minima.clone()]);
                // Actualizamos lista de vertices
                vertices_visitados.push(vertice_visitado);
                if vertices_visitados.contains(&arista_minima.get_vertices().unwrap().0)
                {
                    vertice_visitado = arista_minima.get_vertices().unwrap().1;
                }
                else 
                {
                    vertice_visitado = arista_minima.get_vertices().unwrap().0;    
                }
                // Actualizamos las aristas frontera
                let mut extracted: Vec<&Arista<Vertice, Peso>> = self.lista_aristas.iter()
                        .filter(|x| x.arista_contiene_vertice(vertice_visitado))
                        .collect();
                aristas_frontera.append(&mut extracted);
                for vertice in vertices_visitados.iter()
                {
                    aristas_frontera.retain(|x| !(x.arista_contiene_vertice(vertice)
                                                                        && x.arista_contiene_vertice(vertice_visitado)));
                }
            }

            Some(Arbol::<Vertice, Peso>::new(arbol, vertice_inicial.clone()))
        }

        ///
        /// PRE: Grafo y referencia a Vertice
        /// POST: Terna de Arbol de busqueda de profundidad con la raiz proporcionada y etiquetado.
        /// Si la raiz no esta en el grafo, devuelve None
        /// 
        pub fn arbol_profundidad(&self, v0: &Vertice) -> Option<(Arbol<Vertice, Peso>, Etiquetado<Vertice>)>
        {
            let mut arbol = Self::new();
            let mut df = Etiquetado::new(Some("df"));

            // Definimos una pila para backtracking y un vector de vertices visitados
            let mut backtrack: Vec<&Vertice> = vec![];
            let mut vertices_visitados: Vec<&Vertice> = vec![];
            let mut vertice_visitado = v0;

            // Definimos contador para df
            let mut i: isize = 0;

            while arbol.lista_aristas.len() < self.size() - 1
            {
                // Seleccionamos una arista al azar
                let mut aristas_vecinas: Vec<&Arista<Vertice, Peso>> = self.lista_aristas.iter()
                                                .filter(|x| x.arista_contiene_vertice(vertice_visitado))    
                                                .collect();
                
                for vertice in vertices_visitados.iter().filter(|x| **x != vertice_visitado)
                {
                    aristas_vecinas.retain(|x| !x.arista_contiene_vertice(vertice));
                }

                // Añadimos vertice al etiquetado
                // TODO: Evitar contaminar el etiquetado y el vector de vertices visitados con vertices dobles
                df.add_vertice(vertice_visitado.clone(), i);
                i += 1;
                vertices_visitados.push(vertice_visitado);

                if !aristas_vecinas.is_empty()
                {
                    let arista_seleccionada = *aristas_vecinas.get(0).unwrap();
                    backtrack.push(vertice_visitado);
                    vertice_visitado = arista_seleccionada.other(vertice_visitado).unwrap();
                    arbol.add_aristas(vec![arista_seleccionada.clone()]);
                }
                else 
                {
                    // Si no hay mas opciones para avanzar, usamos la pila
                    vertice_visitado = match backtrack.pop() {
                        Some(e) => e,
                        None => { break; }
                    };
                }
            }

            Some((Arbol::new(arbol, v0.clone()), df))
        }

        ///
        /// PRE: Grafo y vertice desde se va a calcular los caminos minimos
        /// POST: Si el vertice esta en el grafo, terna de Arbol que contiene los caminos minimos y
        /// etiquetado con las longitudes. None si no pertenece al grafo o si faltan pesos
        /// NOTA: Implementacion del algoritmo de Dijkstra. Se requiere que Peso implemente un orden parcial
        /// 
        pub fn arbol_camino_minimo(&self, v0: &Vertice) -> Option<(Arbol<Vertice, Peso>, Etiquetado<Vertice>)>
        where Peso: Ord
        {
            let vertices = self.get_vertices();

            let mut arbol = Self::new();
            let mut distancia = Etiquetado::new(Some("Distancias"));

            // Variables temporales
            let mut distancia_temporal: Vec<Option<Peso>> = vec![None; vertices.len()];
            let mut vertice_visitado = v0;
            let mut acarreo_visitado = Peso::elemento_neutro();
            let mut vertices_visitados: Vec<&Vertice> = vec![];

            let pos_raiz = vertices.iter().position(|x| **x == *v0)?;
            distancia_temporal[pos_raiz] = Some(Peso::elemento_neutro());

            while vertices_visitados.len() < vertices.len() {
                let aristas_vecinas: Vec<&Arista<Vertice, Peso>> = self.lista_aristas.iter()
                                                    .filter(|x| x.arista_contiene_vertice(vertice_visitado))
                                                    .filter(|x| !vertices_visitados.contains(&x.other(vertice_visitado).unwrap()))
                                                    .collect();
                
                for arista in aristas_vecinas.iter()
                {
                    // Comprobamos si el peso es negativo
                    if arista.get_peso().unwrap().es_negativo()
                    {
                        return None;
                    }
                    // Evaluamos las distancias temporales
                    let otro = arista.other(vertice_visitado).unwrap();
                    let otro = vertices.iter().position(|x| **x == *otro).unwrap();
                    let otro = distancia_temporal.get_mut(otro).unwrap();
                    match otro {
                        Some(d) => {
                            let nueva_distancia = acarreo_visitado.suma(arista.get_peso()?);
                            if *d > nueva_distancia
                            {
                                *otro = Some(nueva_distancia);
                            }
                        },
                        None => {
                            *otro = Some(acarreo_visitado.suma(arista.get_peso()?));
                        }
                    }
                }
                // Elegimos el vertice con menor distancia y lo añadimos al arbol
                let menor_distancia = distancia_temporal.iter()
                                            .filter(|x| x.is_some())
                                            .enumerate()
                                            .min_by(|x, y| x.1.cmp(y.1)).unwrap();
                let menor_distancia = (menor_distancia.0, menor_distancia.1.as_ref().unwrap());
                vertices_visitados.push(vertice_visitado);
                vertice_visitado = vertices[menor_distancia.0];
                distancia.add_vertice(vertice_visitado.clone(), menor_distancia.1.to_isize());
                acarreo_visitado = acarreo_visitado.suma(menor_distancia.1);
                let min_arista = aristas_vecinas.into_iter()
                                    .filter(|x: &&Arista<Vertice, Peso>| x.arista_contiene_vertice(vertice_visitado))
                                    .nth(0).unwrap();
                arbol.add_aristas(vec![min_arista.clone()]);
            }
            Some((Arbol::new(arbol, v0.clone()), distancia))
        }
    }

    impl<Vertice, Peso> Clone for Grafo<Vertice, Peso>
    where Vertice: Clone + PartialEq, Peso: PesoT {
        fn clone(&self) -> Self 
        {
            Self {
                lista_aristas: self.lista_aristas.clone()
            }
        }
    }

    pub struct Arbol<Vertice, Peso>
    where Vertice: Clone + PartialEq, Peso: PesoT
    {
        grafo: Grafo<Vertice, Peso>,
        raiz: Vertice
    }

    impl<Vertice, Peso> Arbol<Vertice, Peso> 
    where Vertice: Clone + PartialEq, Peso: PesoT
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
    where Vertice: Clone + PartialEq, Peso: PesoT
    {
        fn clone(&self) -> Self {
            Self{
                grafo: self.grafo.clone(),
                raiz: self.raiz.clone()
            }
        }
    }
}
