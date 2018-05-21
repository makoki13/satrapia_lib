/** TERMINADO */
pub mod almacen {    
    pub use juego::punto::punto::Punto;
    pub use juego::recurso::recurso::Recurso;

    #[derive(Clone)]
    
    pub struct Almacen {
        cantidad:i32,
        id:i32, 
        nombre:String,
        tipo:Recurso,
        posicion: Punto,
        max_cantidad:i32
    }

    impl Almacen {
        pub fn new(tipo_recurso: Recurso, posicion: Punto) -> Almacen {        
            Almacen {
                cantidad: 0,
                id: 0,
                nombre: String::new(),
                tipo: tipo_recurso,
                posicion: posicion,
                max_cantidad: 0
            }
        }

        pub fn add_cantidad(&mut self, cantidad:i32) -> i32 {
            self.cantidad = self.cantidad + cantidad;
            self.get_cantidad()
        }

        pub fn resta_cantidad(&mut self, mut cantidad:i32) -> i32 {
            if cantidad > self.cantidad {
                cantidad = self.cantidad;
                self.cantidad = 0;
            } else {
                self.cantidad = self.cantidad - cantidad;
            }
            cantidad
        }
        
        pub fn get_tipo_recurso(&self) -> Recurso {
            self.tipo.clone()
        }
        
        pub fn get_cantidad(&self) -> i32 {
            self.cantidad
        }

        pub fn get_posicion(&self) -> Punto {
            self.posicion.clone()
        }

        pub fn get_max_cantidad(&self) -> i32 {
            self.max_cantidad
        }

        pub fn get_id(&self) -> i32 {
            self.id
        }

        pub fn get_nombre(&self) -> String {
            self.nombre.clone()
        }

        pub fn get_recurso(&self) -> Recurso {
            self.get_tipo_recurso()
        }
    }
}