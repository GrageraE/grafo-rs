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
    /// Modulo Arista. Componentes:
    ///     Arista: enumerador que representa una arista del grafo
    ///     NoPeso: estructura vacia que representa la imposibilidad de ponderar una arista
    /// 
    pub mod arista;
    pub use arista::arista::NoPeso;
    pub use arista::arista::Arista;

    ///
    /// Submodulo Peso. Componentes:
    ///     PesoT: caracteristica que debe cumplir la ponderacion
    /// 
    pub use arista::arista::PesoT;

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
