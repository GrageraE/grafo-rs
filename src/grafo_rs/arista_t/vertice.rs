///
/// Trait que simplifica la caracteristica comun de los vertices
/// 
pub trait VerticeT : Clone + PartialEq
{}

///
/// PRE: Tipos separados por comas
/// POST: Se implementa VerticeT para los tipos dados. Los tipos deberan implementar `Clone` y `PartialEq`
/// 
#[macro_export]
macro_rules! impl_vertice_t {
    ($($t:ty),+) => {
        $(
            impl VerticeT for $t
            {}
        )+
    };
}

impl_vertice_t!(u32, i32, u8, isize, usize, char);
