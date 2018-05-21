/** TERMINADO */
pub mod edificio {
    pub use juego::punto::punto::Punto;

    #[derive(Clone, Debug)]
    pub enum TipoEdificio { 
        PALACIO, SILOS, CUARTEL, MERCADO, EMBAJADA, TABERNA, CENTRO_DE_INVESTIGACION,
        GRANJA, MINA_DE_ORO, SERRERIA, MINA_DE_HIERRO, CANTERA_DE_PIEDRA,EJERCITO 
    }

    pub struct Edificio {
        id:i32,
        nombre: String,
        tipo:TipoEdificio,
        posicion:Punto,
        coste_construccion:i32,
        tiempo_construccion:i32,
        status:String,
        hayEnvioEnMarcha:bool
    }

    impl Edificio {
        pub fn new(id: i32, nombre: String, tipo: TipoEdificio, posicion:Punto, coste_construccion:i32, tiempo_construccion:i32) -> Edificio {        
            let mensaje = String::from("Sin actividad");
            Edificio {
                id: id,
                nombre: nombre,
                tipo: tipo,
                posicion:posicion,
                coste_construccion: coste_construccion,
                tiempo_construccion: tiempo_construccion,
                status: mensaje,
                hayEnvioEnMarcha: false
            }
        }
    
        pub fn set_coste_construccion(&mut self,cantidad: i32) { self.coste_construccion = cantidad; }
        pub fn set_tiempo_construccion(&mut self,cantidad: i32) { self.tiempo_construccion = cantidad; }
        pub fn get_coste_construccion(&self) -> i32 { self.coste_construccion }
        pub fn get_tiempo_construccion(&self) ->i32 { self.tiempo_construccion }

        pub fn getID(&self) -> i32 { self.id }
        pub fn getTipo(&self) -> TipoEdificio { self.tipo.clone() }
        pub fn getPosicion(&self) -> Punto { self.posicion.clone() }
        pub fn setStatus(&mut self, mensaje:String ) { self.status = mensaje; }
        pub fn getNombre(&self) -> String { self.nombre.clone() }
        pub fn getStatus(&self) -> String { self.status.clone() }
    }
}