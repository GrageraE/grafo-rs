use crate::grafo_rs::{GrafoT, Digrafo, AristaT, Diarista, PesoT, VerticeT};

pub mod flujo;
pub use flujo::Flujo;

pub mod algoritmo;
pub use algoritmo::maximizar_flujo;

#[cfg(test)]
mod tests;

pub struct Red<Vertice, Peso>
where Vertice: VerticeT, Peso: PesoT
{
    nombre: Option<String>,
    flujos: Vec<Flujo<Vertice, Peso>>,
    flujos_fuente: Vec<Flujo<Vertice, Peso>>,
    flujos_sumidero: Vec<Flujo<Vertice, Peso>>
}

///
/// PRE: Red, referencia a arco y valor (opcional - por defecto, 1)
/// 
/// POST: Si el arco esta en la red, se intenta incrementar su flujo. Si es posible, Some(()). Eoc, None
/// 
#[macro_export]
macro_rules! incrementar_flujo {
    ($r:expr, $a:expr) => {
        (|| {
            let flujo_actual = $r.get_valor($a)?;
            $r.set_valor($a, flujo_actual + 1)
        })()
    };
    ($r:expr, $a:expr, $c:expr) => {
        (|| {
            let flujo_actual = $r.get_valor($a)?;
            $r.set_valor($a, flujo_actual + $c)
        })()
    };
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
        sumidero: Vertice, vertices_sumidero: Vec<(Vertice, u64)>, arcos: Vec<(Diarista<Vertice, Peso>, u64)>)
        -> Option<Self>
    {
        // Construimos los arcos
        let arcos_fuente: Vec<(Diarista<Vertice, Peso>, u64)> = vertices_fuente.into_iter()
                            .map(|x| (Diarista::arista_sin_peso(fuente.clone(), x.0), x.1))
                            .collect();
        let arcos_sumidero: Vec<(Diarista<Vertice, Peso>, u64)> = vertices_sumidero.into_iter()
                            .map(|x| (Diarista::arista_sin_peso(x.0, sumidero.clone()), x.1))
                            .collect();
        if arcos_fuente.len() == 0 || arcos_sumidero.len() == 0
        {
            return None;
        }
        // Construimos los arcos
        let flujos_fuente: Vec<Flujo<Vertice, Peso>> = arcos_fuente.into_iter()
                            .map(|x| Flujo::new(x.0, x.1))
                            .collect();
        let flujos_sumidero: Vec<Flujo<Vertice, Peso>> = arcos_sumidero.into_iter()
                            .map(|x| Flujo::new(x.0, x.1))
                            .collect();
        let flujos: Vec<Flujo<Vertice, Peso>> = arcos.into_iter()
                            .map(|x| Flujo::new(x.0, x.1))
                            .collect();
        Some (Self {
            nombre,
            flujos,
            flujos_fuente,
            flujos_sumidero
        })
    }

    ///
    /// PRE: Diarista
    /// POST: Primer flujo con arco coincidente con el dado, si la diarista dada tiene flujo
    /// 
    /// NOTA: Considerese usar [`get_valor`](Red::get_valor) para obtener el valor del flujo
    /// 
    pub fn get_flujo(&self, arco: &Diarista<Vertice, Peso>) -> Option<&Flujo<Vertice, Peso>>
    {
        if arco.arista_contiene_vertice(self.get_fuente())
        {
            return self.flujos_fuente.iter().filter(|x| x.get_arco() == arco).next();
        }
        if arco.arista_contiene_vertice(self.get_sumidero())
        {
            return self.flujos_sumidero.iter().filter(|x| x.get_arco() == arco).next();
        }
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
        if arco.arista_contiene_vertice(self.get_fuente())
        {
            return self.flujos_fuente.iter_mut().filter(|x| x.get_arco() == arco).next();
        }
        if arco.arista_contiene_vertice(self.get_sumidero())
        {
            return self.flujos_sumidero.iter_mut().filter(|x| x.get_arco() == arco).next();
        }
        self.flujos.iter_mut().filter(|x| x.get_arco() == arco).next()
    }

    ///
    /// PRE: Diarista
    /// POST: Si tiene flujo, devuelve su valor. None si no esta en la red
    /// 
    pub fn get_valor(&self, arco: &Diarista<Vertice, Peso>) -> Option<u64>
    {
        Some(self.get_flujo(arco)?.get_valor())
    }

    ///
    /// PRE: Diarista y valor
    /// POST: Si el arco dado tiene flujo, le asigna el valor dado al primero
    /// 
    /// NOTA: No se revisa si el arco esta en la red o si el valor es mayor que su capacidad
    /// 
    pub fn set_valor(&mut self, arco: &Diarista<Vertice, Peso>, valor: u64) -> Option<()>
    {
        self.get_flujo_mut(arco)?.set_valor(valor)
    }

    ///
    /// PRE: Diarista
    /// POST: Diferencia entre capacidad y valor del flujo del arco, si esta en la Red. None eoc
    /// 
    pub fn get_valor_restante(&self, arco: &Diarista<Vertice, Peso>) -> Option<u64>
    {
        let fl = self.get_flujo(arco)?;
        Some(fl.get_valor_restante())
    }

    ///
    /// PRE: Diarista
    /// POST: Si el arco esta en la red, un valor booleano. None eoc
    /// 
    pub fn arco_saturado(&self, arco: &Diarista<Vertice, Peso>) -> Option<bool>
    {
        let fl = self.get_flujo(arco)?;
        Some(fl.saturado())
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
        self.flujos_fuente.get(0).unwrap().get_arco().get_vertices().unwrap().0
    }

    ///
    /// POST: Referencia al sumidero
    /// 
    pub fn get_sumidero(&self) -> &Vertice
    {
        self.flujos_sumidero.get(0).unwrap().get_arco().get_vertices().unwrap().1
    }

    ///
    /// POST: Valor de salida de la red (suma de los valores de los flujos salientes de la fuente)
    /// 
    pub fn get_valor_red(&self) -> u64
    {
        self.flujos_fuente.iter().map(|x| x.get_valor()).sum()
    }

    ///
    /// POST: Vector con referencias a los flujos de la fuente
    /// 
    pub fn get_flujos_fuente(&self) -> Vec<&Flujo<Vertice, Peso>>
    {
        self.flujos_fuente.iter().collect()
    }

    ///
    /// POST: Vector con referencias a los flujos del sumidero
    /// 
    pub fn get_flujos_sumidero(&self) -> Vec<&Flujo<Vertice, Peso>>
    {
        self.flujos_sumidero.iter().collect()
    }

    ///
    /// POST: Vector con referencias a los flujos interiores
    /// 
    pub fn get_flujos(&self) -> Vec<&Flujo<Vertice, Peso>>
    {
        self.flujos.iter().collect()
    }

    ///
    /// POST: Consume la red, devolviendo el digrafo subyacente
    /// 
    pub fn into_digrafo(self) -> Digrafo<Vertice, Peso>
    {
        let mut arcos_fuente: Vec<Diarista<Vertice, Peso>> = self.flujos_fuente.into_iter()
                            .map(|x| x.into_arco()).collect();
        let mut arcos_sumidero: Vec<Diarista<Vertice, Peso>> = self.flujos_sumidero.into_iter()
                            .map(|x| x.into_arco()).collect();
        let mut arcos: Vec<Diarista<Vertice, Peso>> = self.flujos.into_iter()
                            .map(|x| x.into_arco()).collect();

        arcos.append(&mut arcos_fuente);
        arcos.append(&mut arcos_sumidero);
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
            flujos_fuente: self.flujos_fuente.clone(),
            flujos_sumidero: self.flujos_sumidero.clone()
        }
    }
}
