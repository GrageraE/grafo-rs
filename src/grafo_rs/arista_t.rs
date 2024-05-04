pub mod arista_t
{
    pub mod peso;

    pub use peso::peso::PesoT;

    pub use peso::peso::NoPeso;

    pub mod vertice;
    
    pub use vertice::vertice::VerticeT;

    ///
    /// Trait que define operaciones comunes para las aristas
    /// 
    pub trait AristaT<Vertice, Peso> : Clone + PartialEq
    where Vertice: VerticeT, Peso: PesoT
    {
    }

}
