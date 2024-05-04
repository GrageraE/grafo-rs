pub mod vertice
{
    ///
    /// Trait que simplifica la caracteristica comun de los vertices
    /// 
    pub trait VerticeT : Clone + PartialEq
    {}

    impl VerticeT for i32
    {}

    impl VerticeT for u8
    {}

    impl VerticeT for usize 
    {}

    impl VerticeT for isize 
    {}

    impl VerticeT for char
    {}

}
