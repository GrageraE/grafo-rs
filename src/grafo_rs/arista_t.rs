pub mod arista_t
{
    pub mod peso;

    pub use peso::peso::PesoT;

    pub use peso::peso::NoPeso;

    pub mod vertice;
    
    pub use vertice::vertice::VerticeT;

    pub trait AristaT
    {
        fn arista()
    }

}
