/** TERMINADO */
pub mod punto {
    pub use juego::edificio::edificio::{Edificio, TipoEdificio};
    
    #[derive(Clone, Debug)]
    pub enum TipoTerreno { SinDefinir, Prado, Bosque, Mar, AguaPocoProfunda }

    #[derive(Clone, Debug)]
    pub struct Punto {
        x: i32,
        y: i32,
        z: i32,

        tipo_de_terreno:Option<TipoTerreno>,
        region:Option<i32>, // Geografia fisica. No usado
        provincia:Option<i32>, // Geografia politica. No usado
        edificio:Option<TipoEdificio>
    }

    impl Punto {
        pub fn new(x:i32, y:i32, z:i32) -> Punto {        
            Punto {
                x: x,
                y: y,
                z: z,
                tipo_de_terreno: None,
                region: None,
                provincia: None,
                edificio: None
            }
        }

        pub fn new_empty() -> Punto {        
            Punto {
                x: 0,
                y: 0,
                z: 0,
                tipo_de_terreno: None,
                region: None,
                provincia: None,
                edificio: None
            }
        }

        pub fn son_iguales(p:Punto, q:Punto) -> bool  {
            let res:bool =  (p.get_x() == q.get_x()) && (p.get_y() == q.get_y());
            res
        }

        pub fn setAltura (&mut self, z:i32) { self.z = z; }

        pub fn setEdificio(&mut self, e:TipoEdificio) { self.edificio = Some(e); }

        pub fn get_x(&self) -> i32 { self.x }
        pub fn get_y(&self) -> i32 { self.y }
    
        pub fn get_edificio(&self) -> Option<TipoEdificio> { self.edificio.clone() }
        pub fn get_terreno(&self) -> Option<TipoTerreno> { self.tipo_de_terreno.clone() }
    }
}