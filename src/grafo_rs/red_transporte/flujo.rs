use crate::grafo_rs::{Diarista, VerticeT, PesoT};

pub struct Flujo<Vertice, Peso>
where Vertice: VerticeT, Peso: PesoT
{
    arco: Diarista<Vertice, Peso>,
    capacidad: u64,
    valor: u64
}

impl<Vertice, Peso> Flujo<Vertice, Peso>
where Vertice: VerticeT, Peso: PesoT
{
    ///
    /// PRE: Arco y su capacidad
    /// POST: Flujo nulo con capacidad y arco dados
    /// 
    pub fn new(arco: Diarista<Vertice, Peso>, capacidad: u64) -> Flujo<Vertice, Peso>
    {
        Self{
            arco,
            capacidad,
            valor: 0
        }
    }

    ///
    /// POST: Capacidad del flujo
    /// 
    pub fn get_capacidad(&self) -> u64
    {
        self.capacidad
    }

    ///
    /// POST: Valor del flujo
    /// 
    pub fn get_valor(&self) -> u64
    {
        self.valor
    }

    ///
    /// PRE: Nuevo valor
    /// POST: Si el nuevo valor no supera la capacidad, se devuelve Some(()) y se aplica. None eoc. 
    /// 
    pub fn set_valor(&mut self, valor: u64) -> Option<()>
    {
        if valor > self.capacidad {
            return None;
        }
        self.valor = valor;
        Some(())
    }

    ///
    /// POST: Referencia al arco del flujo
    /// 
    pub fn get_arco(&self) -> &Diarista<Vertice, Peso>
    {
        &self.arco
    }

    ///
    /// POST: Consume el flujo, devolviendo su arco
    /// 
    pub fn into_arco(self) -> Diarista<Vertice, Peso>
    {
        self.arco
    }

}

impl<Vertice, Peso> Clone for Flujo<Vertice, Peso>
where Vertice: VerticeT, Peso: PesoT
{
    fn clone(&self) -> Self {
        Self{
            arco: self.arco.clone(),
            capacidad: self.capacidad,
            valor: self.valor
        }
    }
}

impl<Vertice, Peso> PartialEq for Flujo<Vertice, Peso>
where Vertice: VerticeT, Peso: PesoT
{
    fn eq(&self, other: &Self) -> bool {
        self.arco == other.arco && self.capacidad == other.capacidad && self.valor == other.valor
    }
}

impl<Vertice, Peso> PartialOrd for Flujo<Vertice, Peso>
where Vertice: VerticeT, Peso: PesoT
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.valor.partial_cmp(&other.valor)
    }
}
