use crate::grafo_rs::{AristaT, PesoT, VerticeT};

use std::marker::PhantomData;

#[cfg(test)]
mod tests;

pub struct Emparejamiento<'a, Arista, Vertice, Peso>
where Arista: AristaT<Vertice, Peso>, Vertice: VerticeT, Peso: PesoT
{
    aristas: Vec<&'a Arista>,
    v: PhantomData<Vertice>,
    p: PhantomData<Peso>
}

impl<'a, Arista, Vertice, Peso> Emparejamiento<'a, Arista, Vertice, Peso>
where Arista: AristaT<Vertice, Peso>, Vertice: VerticeT, Peso: PesoT
{
    ///
    /// PRE: Lista de referencias a aristas
    /// 
    /// POST: Un emparejamiento, si es valido. None eoc
    /// 
    /// NOTA: Un emparejamiento es valido si no tiene vertices aislados y sus aristas no tienen vertices
    /// en comun
    /// 
    pub fn new(lista: Vec<&'a Arista>) -> Option<Self>
    {
        let mut aristas: Vec<&'a Arista> = vec![];
        let mut vertices: Vec<&Vertice> = vec![];
        for pareja in lista.into_iter()
        {
            let (u, v) = pareja.get_vertices()?;
            if vertices.contains(&u) || vertices.contains(&v) {
                return None;
            }
            vertices.push(u);
            vertices.push(v);
            aristas.push(pareja);
        }
        Some(Self {
            aristas,
            v: PhantomData,
            p: PhantomData
        })
    }

    ///
    /// POST: Devuelve el numero de aristas del emparejamiento
    /// 
    pub fn size(&self) -> usize
    {
        self.aristas.len()
    }

    ///
    /// PRE: Referencia a vertice
    /// 
    /// POST: Si el vertice esta saturado por el emparejamiento, devuelve su arista. Si es libre, None
    /// 
    pub fn buscar_arista(&self, v: &Vertice) -> Option<&Arista>
    {
        self.aristas.iter().find(|x| x.arista_contiene_vertice(v)).copied()
    }

    ///
    /// PRE: Referencia a vertice
    /// 
    /// POST: Si el vertice esta saturado, referencia al vertice contrario. None si es libre
    /// 
    pub fn recorrer(&self, v: &Vertice) -> Option<&Vertice>
    {
        self.buscar_arista(v)?.other(v)
    }

    ///
    /// PRE: Referencia a vertice
    /// 
    /// POST: true si el vertice dado no esta saturado por el emparejamiento. false eoc
    /// 
    pub fn es_libre(&self, v: &Vertice) -> bool
    {
        self.buscar_arista(v).is_none()
    }

    ///
    /// PRE: Arista a añadir
    /// 
    /// POST: true si puede añadirla, false eoc
    /// 
    pub fn agregar_arista(&mut self, e: &'a Arista) -> bool
    {
        let (u, v) = match e.get_vertices() {
            Some((u, v)) => (u, v),
            None => {return false;}
        };
        if !self.es_libre(u) && !self.es_libre(v) {
            self.aristas.push(e);
            return true;
        }
        false
    }

    ///
    /// PRE: Arista
    /// 
    /// POST: Quita la arista del emparejamiento
    /// 
    pub fn quitar_arista(&mut self, e: &Arista)
    where Arista: PartialEq
    {
        self.aristas.retain(|x| *x != e);
    }

}

impl<'a, Arista, Vertice, Peso> PartialEq for Emparejamiento<'a, Arista, Vertice, Peso>
where Arista: AristaT<Vertice, Peso> + PartialEq, Vertice: VerticeT, Peso: PesoT
{
    fn eq(&self, other: &Self) -> bool {
        self.aristas == other.aristas
    }
}

impl<'a, Arista, Vertice, Peso> Clone for Emparejamiento<'a, Arista, Vertice, Peso>
where Arista: AristaT<Vertice, Peso>, Vertice: VerticeT, Peso: PesoT
{
    fn clone(&self) -> Self {
        Self {
            aristas: self.aristas.clone(),
            v: self.v,
            p: self.p
        }
    }
}
