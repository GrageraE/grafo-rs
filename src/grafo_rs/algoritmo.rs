use crate::grafo_rs::{Arbol, Grafo, Arista, 
                    AristaT, PesoT, VerticeT, GrafoT,
                    Etiquetado};

#[cfg(test)]
mod tests;

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
pub fn arbol_peso_minimo<Vertice, Peso>(grafo: &Grafo<Vertice, Peso>) -> Option<Arbol<Grafo<Vertice, Peso>, Vertice, Peso>>
where Vertice: VerticeT, Peso: PesoT + Ord
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

    Some(Arbol::<Grafo<Vertice, Peso>, Vertice, Peso>::from_grafo(arbol, vertice_inicial.clone()))
}

///
/// PRE: Grafo y vertice raiz
/// POST: Arbol de busqueda en anchura en el vertice si esta en el grafo. None eoc
/// 
pub fn arbol_anchura<Vertice, Peso>(grafo: &Grafo<Vertice, Peso>, v0: &Vertice) -> Option<Arbol<Grafo<Vertice, Peso>, Vertice, Peso>>
where Vertice: VerticeT, Peso: PesoT
{
    let mut arbol: Grafo<Vertice, Peso> = Grafo::new();
    // Variables temporales
    let mut vertices_visitados: Vec<&Vertice> = vec![v0];
    let mut aristas_frontera: Vec<&Arista<Vertice, Peso>> = grafo.get_aristas().iter()
                            .filter(|x| x.arista_contiene_vertice(v0))
                            .collect();
    let mut vertices_proximos: Vec<&Vertice> = aristas_frontera.iter()
                            .map(|x| x.other(v0).unwrap())
                            .collect();
    
    while !aristas_frontera.is_empty()
    {
        // Añadimos las aristas frontera
        for (pos, arista) in aristas_frontera.into_iter().enumerate()
        {
            let vertice_contrario = vertices_proximos.get(pos).unwrap();
            if !vertices_visitados.contains(&vertice_contrario)
            {
                vertices_visitados.push(vertice_contrario);
                arbol.add_aristas(vec![arista.clone()]);
            }
        }
        // Actualizamos las aristas frontera
        let mut nuevas_aristas: Vec<&Arista<Vertice, Peso>> = vec![];
        let mut nuevos_vertices: Vec<&Vertice> = vec![];
        for vertice in vertices_proximos.into_iter()
        {
            let mut aristas_vertice: Vec<&Arista<Vertice, Peso>> = grafo.get_aristas().iter()
                            .filter(|x| x.arista_contiene_vertice(vertice))
                            .filter(|x| !vertices_visitados.contains(&x.other(vertice).unwrap()))
                            .collect();
            nuevos_vertices.append(&mut aristas_vertice.iter()
                            .map(|x| x.other(vertice).unwrap()).collect());
            nuevas_aristas.append(&mut aristas_vertice);
        }
        aristas_frontera = nuevas_aristas;
        // Actualizamos los nuevos vertices
        vertices_proximos = nuevos_vertices;
    }
    if arbol.get_aristas().is_empty()
    {
        return None;
    }

    Some(Arbol::from_grafo(arbol, v0.clone()))
}

///
/// PRE: Grafo y referencia a Vertice
/// POST: Terna de Arbol de busqueda de profundidad con la raiz proporcionada y etiquetado.
/// Si la raiz no esta en el grafo, devuelve None
/// 
pub fn arbol_profundidad<Vertice, Peso>(grafo: &Grafo<Vertice, Peso>, v0: &Vertice) -> Option<(Arbol<Grafo<Vertice, Peso>, Vertice, Peso>, Etiquetado<Vertice>)>
where Vertice: VerticeT, Peso: PesoT
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

    Some((Arbol::from_grafo(arbol, v0.clone()), df))
}

///
/// PRE: Grafo y vertice desde se va a calcular los caminos minimos
/// POST: Si el vertice esta en el grafo, terna de Arbol que contiene los caminos minimos y
/// etiquetado con las longitudes. None si no pertenece al grafo o si faltan pesos
/// NOTA: Implementacion del algoritmo de Dijkstra. Se requiere que Peso implemente un orden parcial
/// 
pub fn arbol_camino_minimo<Graf, Vertice, Peso>(grafo: &Graf, v0: &Vertice) -> Option<(Arbol<Graf, Vertice, Peso>, Etiquetado<Vertice>)>
where Graf: GrafoT<Vertice, Peso>, Vertice: VerticeT, Peso: PesoT + Ord
{
    let vertices = grafo.get_vertices();

    let mut arbol = Graf::new();
    let mut distancia = Etiquetado::new(Some("Distancias"));

    // Variables temporales
    let mut distancia_temporal: Vec<(Option<Peso>, &Vertice)> = vec![(None, v0); vertices.len()];
    let mut vertice_visitado = v0;
    let mut acarreo_visitado = Peso::elemento_neutro();
    let mut vertices_visitados: Vec<&Vertice> = vec![v0];

    if !vertices.contains(&v0)
    {
        return None;
    }

    let mut aristas_vecinas: Vec<&Graf::Arista> = grafo.get_aristas().iter()
                                            .filter(|x| x.es_accesible(vertice_visitado))
                                            .filter(|x| !vertices_visitados.contains(&x.other(vertice_visitado).unwrap()))
                                            .collect();
    while !aristas_vecinas.is_empty() {
    
        for arista in aristas_vecinas.iter()
        {
            // Comprobamos si el peso es negativo
            if arista.get_peso()?.es_negativo()
            {
                return None;
            }
            // Evaluamos las distancias temporales
            let otro = arista.other(vertice_visitado).unwrap();
            let otro = vertices.iter().position(|x| **x == *otro).unwrap();
            let otro = distancia_temporal.get_mut(otro).unwrap();
            match &otro.0 {
                Some(d) => {
                    let nueva_distancia = acarreo_visitado.suma(arista.get_peso()?);
                    if d > &nueva_distancia
                    {
                        otro.0 = Some(nueva_distancia);
                        otro.1 = vertice_visitado;
                    }
                },
                None => {
                    otro.0 = Some(acarreo_visitado.suma(arista.get_peso()?));
                    otro.1 = vertice_visitado;
                }
            }
        }
        // Elegimos el vertice con menor distancia y lo añadimos al arbol
        let menor_distancia = distancia_temporal.iter()
                                    .enumerate()
                                    .filter(|x| x.1.0.is_some())
                                    .min_by(|x, y| x.1.0.cmp(&y.1.0)).unwrap();
        let menor_distancia = (menor_distancia.0, menor_distancia.1.0.as_ref().unwrap(), menor_distancia.1.1);
        vertices_visitados.push(vertice_visitado);
        vertice_visitado = vertices[menor_distancia.0];
        distancia.add_vertice(vertice_visitado.clone(), menor_distancia.1.to_isize());
        acarreo_visitado = menor_distancia.1.clone();
        let min_arista = grafo.get_aristas().iter()
                            .filter(|x| x.arista_contiene_vertice(vertice_visitado) && x.arista_contiene_vertice(menor_distancia.2))
                            .min_by(|x, y| x.get_peso().cmp(&y.get_peso())).unwrap();
        arbol.add_aristas(vec![min_arista.clone()]);
        // Lo quitamos del vector de distancia temporal
        let menor_distancia_pos = menor_distancia.0;
        distancia_temporal[menor_distancia_pos] = (None, v0);
        // Actualizamos las aristas vecinas
        aristas_vecinas = grafo.get_aristas().iter()
                                            .filter(|x| x.es_accesible(vertice_visitado))
                                            .filter(|x| !vertices_visitados.contains(&x.other(vertice_visitado).unwrap()))
                                            .collect();
    }
    distancia.add_vertice(v0.clone(), 0);
    Some((Arbol::from_grafo(arbol, v0.clone()), distancia))
}
