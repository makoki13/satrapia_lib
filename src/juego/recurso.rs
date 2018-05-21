/** PENDIENTE */
pub mod recurso {
    #[derive(Clone, Debug)]
    pub enum TipoRecurso { NATURAL, FABRICADO, MILITAR }

    #[derive(Clone, Debug)]        
    pub struct Recurso {
        id: i32,
        nombre: String,        
        tipo: TipoRecurso
    }

    impl Recurso {
        pub fn new(id: i32, nombre: String, tipo: TipoRecurso) -> Recurso {        
            Recurso {
                id: id,
                nombre: nombre,
                tipo: tipo
            }
        }
    
        //use self::TipoRecurso::*;
        //use self::Recurso;

        //pub ORO:Recurso = Recurso::new(1, String::from("ORO"), TipoRecurso::NATURAL); --- Pendiente de implementar
        /*
        Recurso ORO = new Recurso (1, 'ORO', TipoRecurso.NATURAL);
        Recurso POBLACION = new Recurso (2, 'POBLACION', TipoRecurso.MILITAR);
        Recurso COMIDA = new Recurso (3, 'COMIDA', TipoRecurso.NATURAL);
        Recurso MADERA = new Recurso (4, 'MADERA', TipoRecurso.NATURAL);
        Recurso PIEDRA = new Recurso (4, 'PIEDRA', TipoRecurso.NATURAL);
        Recurso HIERRO = new Recurso (5, 'HIERRO', TipoRecurso.NATURAL);
        */

        pub fn get_nombre(&self) -> String { self.nombre.clone() }
    }
}