use crate::grafo_rs::{GrafoT, Digrafo, AristaT, Diarista, PesoT, VerticeT};

pub mod flujo;
pub use flujo::Flujo;

#[cfg(test)]
mod tests;

pub struct Red<Vertice, Peso>
where Vertice: VerticeT, Peso: PesoT
{
    nombre: Option<String>,
    flujos: Vec<Flujo<Vertice, Peso>>,
    fuente: Vertice,
    sumidero: Vertice
}

impl<Vertice, Peso> Red<Vertice, Peso>
where Vertice: VerticeT, Peso: PesoT
{
    ///
    /// PRE:    `nombre`: Nombre de la red
    ///         `fuente`: fuente de la red
    ///         `vertices_fuente`: vertices conectados con la fuente, con sus capacidades
    ///         `sumidero`: sumidero de la red
    ///         `vertices_sumidero`: vertices conectados con el sumidero, con sus capacidades
    ///         `arcos`: resto de los arcos de la red, con sus capacidades. No hace falta incluir los anteriores
    /// 
    /// POST: Red de transporte
    /// 
    pub fn new(nombre: Option<String>, fuente: Vertice, vertices_fuente: Vec<(Vertice, u64)>, 
        sumidero: Vertice, vertices_sumidero: Vec<(Vertice, u64)>, mut arcos: Vec<(Diarista<Vertice, Peso>, u64)>)
        -> Self
    {
        // Recogemos las aristas
        let mut arcos_capacidad: Vec<(Diarista<Vertice, Peso>, u64)> = vec![];
        let mut vertices_fuente = vertices_fuente.into_iter()
                        .map(|x| (Diarista::arista_sin_peso(fuente.clone(), x.0), x.1))
                        .collect();
        let mut vertices_sumidero = vertices_sumidero.into_iter()
                        .map(|x| (Diarista::arista_sin_peso(sumidero.clone(), x.0), x.1))
                        .collect();

        arcos_capacidad.append(&mut vertices_fuente);
        arcos_capacidad.append(&mut vertices_sumidero);
        arcos_capacidad.append(&mut arcos);
        // Construimos los flujos
        let flujos: Vec<Flujo<Vertice, Peso>> = arcos_capacidad.into_iter()
                        .map(|x| Flujo::new(x.0, x.1))
                        .collect();
        Self{
            nombre,
            flujos,
            fuente,
            sumidero
        }
    }

    ///
    /// PRE: Diarista
    /// POST: Primer flujo con arco coincidente con el dado, si la diarista dada tiene flujo
    /// 
    /// NOTA: Considerese usar [`get_valor`](Red::get_valor) para obtener el valor del flujo
    /// 
    pub fn get_flujo(&self, arco: &Diarista<Vertice, Peso>) -> Option<&Flujo<Vertice, Peso>>
    {
        self.flujos.iter().filter(|x| x.get_arco() == arco).next()
    }

    ///
    /// PRE: Diarista
    /// POST: Primer flujo con arco coincidente con el dado, si la diarista dada tiene flujo
    /// 
    /// NOTA: Esta es la version mutable de [`get_flujo`](Red::get_flujo)
    /// 
    /// NOTA: Considerese usar [`set_valor`](Red::set_valor) para cambiar el valor del flujo
    /// 
    pub fn get_flujo_mut(&mut self, arco: &Diarista<Vertice, Peso>) -> Option<&mut Flujo<Vertice, Peso>>
    {
        self.flujos.iter_mut().filter(|x| x.get_arco() == arco).next()
    }

    ///
    /// PRE: Diarista
    /// POST: Si tiene flujo, devuelve su valor. None si no esta en la red
    /// 
    pub fn get_valor(&self, arco: &Diarista<Vertice, Peso>) -> Option<u64>
    {
        Some(self.flujos.iter().filter(|x| x.get_arco() == arco).next()?.get_valor())
    }

    ///
    /// PRE: Diarista y valor
    /// POST: Si el arco dado tiene flujo, le asigna el valor dado al primero
    /// 
    /// NOTA: No se revisa si el arco esta en la red o si el valor es mayor que su capacidad
    /// 
    pub fn set_valor(&mut self, arco: &Diarista<Vertice, Peso>, valor: u64)
    {
        if let Some(f) = 
            self.flujos.iter_mut().filter(|x| x.get_arco() == arco).next()
        {
            f.set_valor(valor);
        } 
    }
    ///
    /// POST: Nombre de la red
    /// 
    pub fn get_nombre(&self) -> Option<&str>
    {
        self.nombre.as_deref()
    }

    ///
    /// POST: Referencia a la fuente
    /// 
    pub fn get_fuente(&self) -> &Vertice
    {
        &self.fuente
    }

    ///
    /// POST: Referencia al sumidero
    /// 
    pub fn get_sumidero(&self) -> &Vertice
    {
        &self.sumidero
    }

    ///
    /// POST: Consume la red, devolviendo el digrafo subyacente
    /// 
    pub fn into_digrafo(self) -> Digrafo<Vertice, Peso>
    {
        let arcos: Vec<Diarista<Vertice, Peso>> = self.flujos.into_iter()
                            .map(|x| x.into_arco()).collect();
        Digrafo::from_aristas(arcos)
    }

}

impl<Vertice, Peso> Clone for Red<Vertice, Peso>
where Vertice: VerticeT, Peso: PesoT
{
    fn clone(&self) -> Self {
        Self {
            nombre: self.nombre.clone(),
            flujos: self.flujos.clone(),
            fuente: self.fuente.clone(),
            sumidero: self.sumidero.clone()
        }
    }
}
