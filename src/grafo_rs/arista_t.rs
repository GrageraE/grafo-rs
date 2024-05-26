pub mod peso;

pub use peso::{NoPeso, PesoT};

pub mod vertice;

pub use vertice::VerticeT;

///
/// Trait que define operaciones comunes para las aristas
/// 
pub trait AristaT<Vertice, Peso> : Clone + PartialEq
where Vertice: VerticeT, Peso: PesoT
{
    ///
    /// Crea una arista a partir de dos vertices con un posible Peso p
    /// 
    fn arista(v: Vertice, w: Vertice, p: Option<Peso>) -> Self;

    ///
    /// Crea una arista sin peso a partir de dos vertices
    /// 
    fn arista_sin_peso(v: Vertice, w: Vertice) -> Self;

    ///
    /// Crea un vertice aislado
    /// 
    fn vertice(v: Vertice) -> Self;

    ///
    /// PRE: Arista
    /// POST: Una arista formada por los vertices de la anterior con peso = elemento neutro, si es arista. 
    /// None eoc
    /// 
    fn peso_por_defecto(&self) -> Option<Self>
    {
        let (v1, v2) = self.get_vertices()?;
        Some(Self::arista(v1.clone(), v2.clone(), Some(Peso::elemento_neutro())))
    }

    ///
    /// Funcion miembro. Devuelve true si es una arista y tiene como extremo v0. False eoc
    /// 
    fn arista_contiene_vertice(&self, v0: &Vertice) -> bool;

    ///
    /// Funcion miembro. Devuelve true si la arista es recorrible por el vertice v0. False eoc
    /// 
    fn es_accesible(&self, v0: &Vertice) -> bool
    {
        self.arista_contiene_vertice(v0)
    }

    ///
    /// Funcion miembro. Devuelve una tupla de sus extremos si es una arista. None eoc
    /// 
    fn get_vertices(&self) -> Option<(&Vertice, &Vertice)>;

    ///
    /// Funcion miembro. Consume la arista y devuelve sus vertices. None si es vertice aislado
    /// 
    fn into_vertices(self) -> Option<(Vertice, Vertice)>;

    ///
    /// Funcion miembro. Devuelve true si es un vertice aislado y v es su vertice. False eoc
    /// 
    fn es_vetice_aislado(&self, v: &Vertice) -> bool;

    ///
    /// Funcion miembro. Si es una arista devuelve su posible Peso. None si es vertice aislado
    /// 
    fn get_peso(&self) -> Option<&Peso>;

    ///
    /// PRE: Referencia a vertice
    /// POST: Si es una arista que contiene al vertice, devuelve la otra. None eoc.
    /// 
    fn other(&self, v: &Vertice) -> Option<&Vertice>
    {
        let (v1, v2) = self.get_vertices()?;
        if v == v1
        {
            return Some(v2);
        }
        if v == v2
        {
            return Some(v1);
        }
        None
    }

    ///
    /// PRE: Referencia a lista de aristas
    /// POST: Referencia a Arista con peso minimo. Si la lista es vacia, None.
    /// NOTA: Requiere que el Peso tenga un orden parcial definido. Puede usarse la funcion peso_por_defecto.
    /// 
    // TODO: Mejorar modelo de ownership de aristas para evitar copias innecesarias
    fn min_aristas(aristas: Vec<&Self>) -> Option<&Self>
    where Peso: PartialOrd
    {
        let mut min = aristas.get(0)?;
        for i in 1..aristas.len()
        {
            let arista_actual = aristas.get(i).unwrap();
            min = match arista_actual.get_peso()?.partial_cmp(min.get_peso().unwrap())? {
                std::cmp::Ordering::Less => arista_actual,
                _ => min
            };
        }
        Some(min)
    }

    ///
    /// PRE: Vector con referencias a Aristas con Peso = isize
    /// POST: Si todas tienen peso devuelve la suma de los pesos. None eoc
    /// 
    fn sumatorio_pesos(aristas: &Vec<Self>) -> Peso
    {
        let pesos: Vec<&Peso> = aristas.into_iter()
            .filter_map(|x| x.get_peso())
            .collect();
        let mut suma = Peso::elemento_neutro();
        for peso in pesos.into_iter()
        {
            suma = suma.suma(peso);
        }
        suma
    }    
}

