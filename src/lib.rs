///
/// Modulo grafo_rs
/// Componente raiz de la libreria
/// 
pub mod grafo_rs
{
    ///
    ///  Modulo Grafo_T. Componentes:
    ///     GrafoT: caracteristica comun de grafo
    /// 
    pub mod grafo_t;
    pub use grafo_t::GrafoT;

    ///
    /// Modulo Grafo. Componentes:
    ///     Grafo: estructura que representa un Grafo
    /// 
    pub mod grafo;
    pub use grafo::Grafo;

    ///
    /// Modulo Arbol. Componentes:
    ///     Arbol: estructura envoltorio que asegura que el grafo contenido es un arbol
    /// 
    pub mod arbol;
    pub use arbol::Arbol;

    ///
    /// Modulo Digrafo. Componentes:
    ///     Digrafo: estructura que representa un Digrafo
    /// 
    pub mod digrafo;
    pub use digrafo::Digrafo;

    ///
    /// Modulo Arista_T. Componentes:
    ///     AristaT: caracteristica comun que deben cumplir las aristas
    /// 
    pub mod arista_t;
    pub use arista_t::AristaT;

    ///
    /// Submodulo Peso de Arista_T. Componentes:
    ///     PesoT: caracteristica que debe cumplir la ponderacion
    ///     NoPeso: estructura vacia que representa la imposibilidad de ponderar una arista
    /// 
    pub use arista_t::PesoT;
    pub use arista_t::NoPeso;

    ///
    /// Submodulo Vertice de Arista_T. Componentes:
    ///     VerticeT: caracteristica que deben cumplir los vertices
    /// 
    pub use arista_t::VerticeT;

    ///
    /// Modulo Arista. Componentes:
    ///     Arista: enumerador que representa una arista del grafo
    /// 
    pub mod arista;
    pub use arista::Arista;

    ///
    /// Modulo Diarista. Componentes:
    ///     Diarista: enumerador que representa una arista dirigida del grafo
    /// 
    pub mod diarista;
    pub use diarista::Diarista;

    ///
    /// Modulo Algoritmo.
    /// 
    pub mod algoritmo;
    pub use algoritmo::*;

    ///
    /// Modulo Red_Transporte. Componentes:
    ///     Red: Red de transporte
    ///     Flujo: El flujo de una Diarista
    /// 
    pub mod red_transporte;
    pub use red_transporte::Red;
    pub use red_transporte::Flujo;
    pub use red_transporte::maximizar_flujo;
    
    ///
    /// Modulo Etiquetado. Componentes:
    ///     Etiqueta: estructura que contiene un vertice y su etiqueta (usize)
    ///     Etiquetado: estructura envoltorio que contiene un vector de Etiqueta
    /// 
    pub mod etiquetado;
    pub use etiquetado::Etiqueta;
    pub use etiquetado::Etiquetado;

}
