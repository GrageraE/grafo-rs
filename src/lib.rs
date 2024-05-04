///
/// Modulo grafo_rs
/// Componente raiz de la libreria
/// 
pub mod grafo_rs
{
    ///
    /// Modulo Grafo. Componentes:
    ///     Grafo: estructura que representa un Grafo
    ///     Arbol: estructura envoltorio que asegura que el grafo contenido es un arbol
    /// 
    pub mod grafo;
    pub use grafo::grafo::Grafo;
    pub use grafo::grafo::Arbol;

    ///
    /// Modulo Arista_T. Componentes:
    ///     AristaT: caracteristica comun que deben cumplir las aristas
    /// 
    pub mod arista_t;
    pub use arista_t::arista_t::AristaT;

    ///
    /// Submodulo Peso de Arista_T. Componentes:
    ///     PesoT: caracteristica que debe cumplir la ponderacion
    ///     NoPeso: estructura vacia que representa la imposibilidad de ponderar una arista
    /// 
    pub use arista_t::arista_t::PesoT;
    pub use arista_t::arista_t::NoPeso;

    ///
    /// Submodulo Vertice de Arista_T. Componentes:
    ///     VerticeT: caracteristica que deben cumplir los vertices
    /// 
    pub use arista_t::arista_t::VerticeT;

    ///
    /// Modulo Arista. Componentes:
    ///     Arista: enumerador que representa una arista del grafo
    /// 
    pub mod arista;
    pub use arista::arista::Arista;

    ///
    /// Modulo Diarista
    /// 
    pub mod diarista;

    ///
    /// Modulo Algoritmo.
    /// 
    pub mod algoritmo;
    pub use algoritmo::algoritmo::*;
    
    ///
    /// Modulo Etiquetado. Componentes:
    ///     Etiqueta: estructura que contiene un vertice y su etiqueta (usize)
    ///     Etiquetado: estructura envoltorio que contiene un vector de Etiqueta
    /// 
    pub mod etiquetado;
    pub use etiquetado::etiquetado::Etiqueta;
    pub use etiquetado::etiquetado::Etiquetado;

}
