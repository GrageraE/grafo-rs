use crate::{grafo_rs::{AristaT, Diarista, PesoT, VerticeT}, incrementar_flujo};

use super::Red;

#[cfg(test)]
mod tests;

///
/// Funcion auxiliar que encuentra un camino de F-aumento en la red, si existe.
/// 
/// Implementa el algoritmo de Edmonds-Karp
/// 
fn encontrar_camino_aumento<Vertice, Peso>(red: &Red<Vertice, Peso>) 
    -> Option<Vec<&Diarista<Vertice, Peso>>>
where Vertice: VerticeT, Peso: PesoT
{
    // Listas de arcos no saturados de la red
    let arcos_iniciales: Vec<&Diarista<Vertice, Peso>> = red.get_flujos_fuente().iter()
                            .filter_map(|x| {
                                match !x.saturado() {
                                    true => Some(x.get_arco()),
                                    false => None
                                }
                            }).collect();
    let mut arcos_intermedios: Vec<&Diarista<Vertice, Peso>> = red.get_flujos().iter()
                            .filter_map(|x| {
                                match !x.saturado() {
                                    true => Some(x.get_arco()),
                                    false => None
                                }
                            }).collect();
    let mut arcos_finales: Vec<&Diarista<Vertice, Peso>> = red.get_flujos_sumidero().iter()
                            .filter_map(|x| {
                                match !x.saturado() {
                                    true => Some(x.get_arco()),
                                    false => None
                                }
                            }).collect();
    let arcos = {arcos_intermedios.append(&mut arcos_finales); arcos_intermedios};
    /*
     * Corremos el algoritmo de busqueda en anchura para encontrar un camino de F-aumento.
     * Incluiremos, junto con el arco, una referencia al arco anterior
     *                              (arco, arco_anterior)
     */
    let mut aristas_frontera: Vec<(&&Diarista<Vertice, Peso>, &&Diarista<Vertice, Peso>)> = arcos.iter()
                    .filter_map(|x|
                        Some((x, arcos_iniciales.iter().find(
                            |v| v.get_vertices().unwrap().1 == x.get_vertices().unwrap().0)?))
                    )
                    .collect();
    // Variables temporales
    let mut vertices_visitados: Vec<&Vertice> = aristas_frontera.iter()
                    .map(|x| x.0.get_vertices().unwrap().0)
                    .collect();
    let mut arcos_recorridos: Vec<(&&Diarista<Vertice, Peso>, &&Diarista<Vertice, Peso>)> = vec![];   
    
    let mut ultima_arista: Option<(&&Diarista<Vertice, Peso>, &&Diarista<Vertice, Peso>)> = None;
    while !aristas_frontera.is_empty()
    {
        let mut arcos_agregados: Vec<&&Diarista<Vertice, Peso>> = vec![];
        // Añadimos las aristas frontera
        for (arista, arista_anterior) in aristas_frontera.into_iter()
        {
            let vertice_contrario = &arista.get_vertices().unwrap().1;
            if !vertices_visitados.contains(vertice_contrario)
            {
                vertices_visitados.push(vertice_contrario);
                arcos_recorridos.push((arista, arista_anterior));
                arcos_agregados.push(arista);
                if *vertice_contrario == red.get_sumidero()
                {
                    ultima_arista = Some((arista, arista_anterior));
                }
            }
        }
        if ultima_arista.is_some()
        {
            break;
        }
        // Actualizamos las aristas frontera
        let mut nuevas_aristas: Vec<(&&Diarista<Vertice, Peso>, &&Diarista<Vertice, Peso>)> = vec![];
        let mut nuevos_vertices: Vec<&Vertice> = vec![];
        for arco in arcos_agregados.into_iter()
        {
            let vertice = arco.get_vertices().unwrap().1;
            let mut aristas_vertice: Vec<(&&Diarista<Vertice, Peso>, &&Diarista<Vertice, Peso>)> = arcos.iter()
                        .filter(|x| x.es_accesible(vertice))
                        .filter(|x| !vertices_visitados.contains(&x.other(vertice).unwrap()))
                        .map(|x| (x, arco))
                        .collect();
            nuevos_vertices.append(&mut aristas_vertice.iter()
                        .map(|x| x.0.other(vertice).unwrap()).collect());
            nuevas_aristas.append(&mut aristas_vertice);
        }
        aristas_frontera = nuevas_aristas;
        // Actualizamos los nuevos vertices
    }
    let ultima_arista = ultima_arista?;
    /*
     * Si llegamos a aqui, hemos encontrado un camino de F-aumento.
     * Se procede a deshilar el camino
     */
    let mut camino: Vec<&Diarista<Vertice, Peso>> = vec![];
    let mut arista = ultima_arista;
    while !arcos_iniciales.contains(arista.1)
    {
        camino.insert(0, arista.0);
        arista = *arcos_recorridos.iter().find(|x| x.0 == arista.1).unwrap();
    }
    camino.insert(0, arista.0);
    camino.insert(0, arista.1);
    Some(camino)
}

///
/// PRE: Red mutable
/// 
/// POST: La red se maximiza
/// 
/// NOTA: Implementacion del algoritmo de Edmods-Karp
/// 
pub fn maximizar_flujo<Vertice, Peso>(red: &mut Red<Vertice, Peso>)
where Vertice: VerticeT, Peso: PesoT
{
    // Buscamos caminos de F-aumento
    let mut camino_aumento = encontrar_camino_aumento(red);
    while let Some(camino) = camino_aumento
    {
        let camino: Vec<Diarista<Vertice, Peso>> = camino.into_iter().cloned().collect();
        // Calculamos el flujo que tenemos que incrementar
        let aumento = camino.iter()
                        .map(|x| red.get_valor_restante(x).unwrap())
                        .min().unwrap();
        // Incrementamos el flujo de los arcos
        for arco in camino.into_iter()
        {
            incrementar_flujo!(red, &arco, aumento);
        }
        camino_aumento = encontrar_camino_aumento(red);
    }
}
