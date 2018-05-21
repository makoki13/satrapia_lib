/** TERMINADO */
pub mod productor {
    pub use juego::recurso::recurso::Recurso;
    pub use juego::punto::punto::Punto;

    #[derive(Clone)]
    pub struct Productor {
        posicion: Punto,
        recurso: Recurso,
        stock: i32,
        cantidad_maxima: i32,
        ratio_produccion: i32
    }

    impl Productor {
        pub fn new(posicion: Punto, recurso: Recurso, mut cantidad_inicial: i32, cantidad_maxima: i32, ratio_produccion: i32) -> Productor {        
            if cantidad_inicial > cantidad_maxima  {cantidad_inicial = cantidad_maxima; }
            Productor {
                posicion: posicion,
                recurso: recurso,
                stock: cantidad_inicial,
                cantidad_maxima: cantidad_maxima,
                ratio_produccion: ratio_produccion
            }
        }

        pub fn extrae (&mut self, mut cantidad: i32 ) -> i32  {
            cantidad = cantidad * self.ratio_produccion; // Para penalizaciones y bonus
            // Los productores con cantidadMaxima = 0 son inagotables.
            if self.cantidad_maxima == 0 { return cantidad; }
            if cantidad > self.stock {
                cantidad = self.stock;
                self.stock = 0;
            } 
            else {                
                self.stock = self.stock - cantidad;
            }
            cantidad
        }

        pub fn getStock(&self) -> i32 { return self.stock; }
    }
}