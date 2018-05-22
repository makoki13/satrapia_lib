/** TERMINADO */
pub mod extractor {
    pub use juego::almacen::almacen::Almacen;
    pub use juego::productor::productor::Productor;

    #[derive(Clone)]
    pub struct Extractor {
        productor: Productor,
        almacen: Almacen,
        cantidad: i32
    }

    impl Extractor {
        pub fn new(productor: Productor, almacen: Almacen, cantidad: i32) -> Extractor {        
            Extractor {
                productor: productor,
                almacen: almacen,
                cantidad: cantidad
            }
        }

        pub fn get_cantidad (&mut self) -> i32 {   
            //println!("Cantidad extractor: {}", self.cantidad);         
            self.productor.extrae(self.cantidad)
        }
    }
}