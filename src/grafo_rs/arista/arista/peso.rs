pub mod peso
{
    ///
    /// Trait que define las operaciones necesarias para el Peso de las aristas
    /// 
    pub trait PesoT: Clone + PartialEq
    {
        ///
        /// Elemento neutro del Peso
        /// 
        fn elemento_neutro() -> Self;

        ///
        /// Operacion para sumar Pesos
        /// 
        fn suma(&self, otro: &Self) -> Self;

        ///
        /// Determinar si el Peso es negativo
        /// 
        fn es_negativo(&self) -> bool;

        ///
        /// Operacion para convertir a isize para usar etiquetado
        /// NOTA: Hay disponible una implementacion por defecto
        /// 
        fn to_isize(&self) -> isize
        {
            0
        }
    }

    impl PesoT for i32
    {
        fn elemento_neutro() -> Self {
            0
        }

        fn suma(&self, otro: &Self) -> Self {
            self + otro
        }

        fn es_negativo(&self) -> bool {
            self < &0
        }

        fn to_isize(&self) -> isize {
            *self as isize
        }
    }

    impl PesoT for u8
    {
        fn elemento_neutro() -> Self {
            0
        }

        fn es_negativo(&self) -> bool {
            false
        }

        fn suma(&self, otro: &Self) -> Self {
            self + otro
        }

        fn to_isize(&self) -> isize {
            *self as isize
        }
    }

    impl PesoT for usize {
        fn elemento_neutro() -> Self {
            0
        }

        fn suma(&self, otro: &Self) -> Self {
            self + otro
        }

        fn es_negativo(&self) -> bool {
            false
        }

        fn to_isize(&self) -> isize {
            *self as isize
        }
    }

    impl PesoT for isize {
        fn elemento_neutro() -> Self {
            0
        }

        fn es_negativo(&self) -> bool {
            self < &0
        }

        fn suma(&self, otro: &Self) -> Self {
            self + otro
        }

        fn to_isize(&self) -> isize {
            *self
        }
    }
}