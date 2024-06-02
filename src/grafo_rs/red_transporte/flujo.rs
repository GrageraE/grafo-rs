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
    pub fn new(arco: Diarista<Vertice, Peso>, capacidad: u64) -> Flujo<Vertice, Peso>
    {
        Self{
            arco,
            capacidad,
            valor: 0
        }
    }

    pub fn get_capacidad(&self) -> u64
    {
        self.capacidad
    }

    pub fn get_valor(&self) -> u64
    {
        self.valor
    }

    pub fn set_valor(&mut self, valor: u64) -> Option<()>
    {
        if valor > self.capacidad {
            return None;
        }
        self.valor = valor;
        Some(())
    }

    pub fn get_arco(&self) -> &Diarista<Vertice, Peso>
    {
        &self.arco
    }

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
