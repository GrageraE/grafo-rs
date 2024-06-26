use crate::grafo_rs::{AristaT, NoPeso, PesoT, VerticeT, Arista};

pub enum Diarista<Vertice, Peso = NoPeso>
where Vertice: VerticeT, Peso: PesoT
{
    Diarista(Vertice, Vertice, Option<Peso>),
    VerticeAislado(Vertice)
}

impl<Vertice, Peso> Diarista<Vertice, Peso>
where Vertice: VerticeT, Peso: PesoT
{
    pub fn into_arista(self) -> Arista<Vertice, Peso>
    {
        match self {
            Self::Diarista(u, v, p) => Arista::arista(u, v, p),
            Self::VerticeAislado(u) => Arista::vertice(u)
        }
    }
}

impl<Vertice, Peso> AristaT<Vertice, Peso> for Diarista<Vertice, Peso>
where Vertice: VerticeT, Peso: PesoT
{
    fn arista(v: Vertice, w: Vertice, p: Option<Peso>) -> Self 
    {
        Self::Diarista(v, w, p)    
    }

    fn arista_sin_peso(v: Vertice, w: Vertice) -> Self {
        Self::Diarista(v, w, None)
    }

    fn vertice(v: Vertice) -> Self 
    {
        Self::VerticeAislado(v)
    }

    fn arista_contiene_vertice(&self, v0: &Vertice) -> bool {
        match &self {
            Self::Diarista(u, v, _) => u == v0 || v == v0,
            _ => false
        }
    }

    ///
    /// PRE: Vertice
    /// POST: True si es una arista y su primer extremo en v0. False eoc
    /// 
    fn es_accesible(&self, v0: &Vertice) -> bool {
        // Es accesible si es una arista y v0 es su primer extremo
        match &self {
            Diarista::Diarista(u, _, _) => v0 == u,
            _ => false
        }
    }

    fn es_vetice_aislado(&self, v: &Vertice) -> bool {
        match &self {
            Self::VerticeAislado(v0) => v == v0,
            _ => false
        }
    }

    fn get_vertices(&self) -> Option<(&Vertice, &Vertice)> {
        if let Self::Diarista(u, v, _) = &self
        {
            return Some((u, v));
        }
        None
    }

    fn into_vertices(self) -> Option<(Vertice, Vertice)> 
    {
        if let Diarista::Diarista(v, w, _) = self
        {
            return Some((v, w));
        }
        None
    }

    fn get_peso(&self) -> Option<&Peso> {
        match &self {
            Self::Diarista(_, _, p) => p.as_ref(),
            _ => None
        }
    }

}

impl<Vertice, Peso> Clone for Diarista<Vertice, Peso>
where Vertice: VerticeT, Peso: PesoT
{
    fn clone(&self) -> Self {
        match &self {
            Self::Diarista(v, w, p) 
                => Self::Diarista(v.clone(), w.clone(), p.clone()),
            Self::VerticeAislado(v) 
                => Self::VerticeAislado(v.clone())
        }
    }
}

impl<Vertice, Peso> PartialEq for Diarista<Vertice, Peso>
where Vertice: VerticeT, Peso: PesoT
{
    fn eq(&self, other: &Self) -> bool {
        match &self {
            Self::Diarista(v0, w0, p0) => {
                if let Self::Diarista(v1, w1, p1) = other
                {
                    // La arista no es simetrica: deben coincidir los extremos en orden
                    return v0 == v1 && w0 == w1 && p0 == p1;
                }
                return false;
            },
            Self::VerticeAislado(v0) => {
                if let Self::VerticeAislado(v1) = other
                {
                    return v0 == v1;
                }
                return false;
            }
        }
    }
}
