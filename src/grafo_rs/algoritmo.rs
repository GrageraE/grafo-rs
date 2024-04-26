pub mod algoritmo
{
    use crate::grafo_rs::Grafo;
    use crate::grafo_rs::Arbol;

    use crate::grafo_rs::Arista;
    
    use crate::grafo_rs::PesoT;
    
    use crate::grafo_rs::Etiquetado;
    
    ///
    /// PRE: Cierto
    /// POST: Arbol generador de peso minimo
    /// NOTA: Implementacion del algoritmo de Prim. Requere que el Peso tenga un orden parcial definido
    /// 
    pub fn arbol_peso_minimo<Vertice, Peso>(grafo: &Grafo<Vertice, Peso>) -> Option<Arbol<Vertice, Peso>>
    where Vertice: Clone + PartialEq, Peso: PesoT + Ord
    {
        let mut arbol = Grafo::new();
        // Seleccionamos un vertice aleatorio
        let vertice_inicial;
        if let Some(e) = grafo.get_aristas().get(0)
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
        let mut aristas_frontera: Vec<&Arista<Vertice, Peso>> = grafo.get_aristas().iter()
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
            let mut extracted: Vec<&Arista<Vertice, Peso>> = grafo.get_aristas().iter()
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
    pub fn arbol_profundidad<Vertice, Peso>(grafo: &Grafo<Vertice, Peso>, v0: &Vertice) -> Option<(Arbol<Vertice, Peso>, Etiquetado<Vertice>)>
    where Vertice: Clone + PartialEq, Peso: PesoT
    {
        let mut arbol = Grafo::new();
        let mut df = Etiquetado::new(Some("df"));

        // Definimos una pila para backtracking y un vector de vertices visitados
        let mut backtrack: Vec<&Vertice> = vec![];
        let mut vertices_visitados: Vec<&Vertice> = vec![];
        let mut vertice_visitado = v0;

        // Definimos contador para df
        let mut i: isize = 0;

        while arbol.get_aristas().len() < grafo.size() - 1
        {
            // Seleccionamos una arista al azar
            let mut aristas_vecinas: Vec<&Arista<Vertice, Peso>> = grafo.get_aristas().iter()
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
    pub fn arbol_camino_minimo<Vertice, Peso>(grafo: &Grafo<Vertice, Peso>, v0: &Vertice) -> Option<(Arbol<Vertice, Peso>, Etiquetado<Vertice>)>
    where Vertice: Clone + PartialEq, Peso: PesoT + Ord
    {
        let vertices = grafo.get_vertices();

        let mut arbol = Grafo::new();
        let mut distancia = Etiquetado::new(Some("Distancias"));

        // Variables temporales
        let mut distancia_temporal: Vec<Option<Peso>> = vec![None; vertices.len()];
        let mut vertice_visitado = v0;
        let mut acarreo_visitado = Peso::elemento_neutro();
        let mut vertices_visitados: Vec<&Vertice> = vec![];

        let pos_raiz = vertices.iter().position(|x| **x == *v0)?;
        distancia_temporal[pos_raiz] = Some(Peso::elemento_neutro());

        while vertices_visitados.len() < vertices.len() {
            let aristas_vecinas: Vec<&Arista<Vertice, Peso>> = grafo.get_aristas().iter()
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