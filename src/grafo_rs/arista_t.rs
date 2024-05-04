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
        fn arista(v: Vertice, w: Vertice, p: Option<Peso>) -> Self;

        fn arista_sin_peso(v: Vertice, w: Vertice) -> Self;

        fn vertice(v: Vertice) -> Self;

        fn peso_por_defecto<T>(&self) -> Option<T>
        where T: AristaT<Vertice, u8>;

        fn arista_contiene_vertice(&self, v0: &Vertice) -> bool;

        fn get_vertices(&self) -> Option<(&Vertice, &Vertice)>;

        fn es_vetice_aislado(&self, v: &Vertice) -> bool;

        fn get_peso(&self) -> Option<&Peso>;

        fn other(&self, v: &Vertice) -> Option<&Vertice>;

        fn min_aristas(aristas: Vec<&Self>) -> Option<&Self>
        where Peso: PartialOrd;

        fn sumatorio_pesos(aristas: &Vec<Self>) -> Peso;
    }

}
