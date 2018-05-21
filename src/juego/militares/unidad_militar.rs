pub mod unidad_militar {
    pub use juego::recurso::recurso::Recurso;

    pub struct Unidad_Militar {
        recurso: Option<Recurso>
    }

    impl Unidad_Militar {

        pub fn new() -> Unidad_Militar {
            Unidad_Militar {
                recurso: None
            }
        }
    }
 
    //use self::Unidad_Militar;

    pub mod civil_con_honda {
        use juego::militares::unidad_militar::unidad_militar::Unidad_Militar;

        pub struct Civil_Con_Honda {
            recurso: Option<Unidad_Militar>
        }   

        impl Civil_Con_Honda {            
            pub fn new() -> Civil_Con_Honda {
                Civil_Con_Honda {
                    recurso: None
                }
            }

            pub fn _test (&self) -> String  { String::from("TST")}
        }
    }
}

/*
class UnidadMilitar extends Recurso {
  num _danyoRecibido = 0;

  num id;
  String nombre;
  num _vidaInicial;
  num _danyoInflingido;
  num _fuerza;
  num _moral;
  num _costeUnitario;
  num maxUnidadesEnEntrenamiento;

  UnidadMilitar (this.id, this.nombre, this._vidaInicial, this._danyoInflingido, this._fuerza, this._moral, this._costeUnitario, this.maxUnidadesEnEntrenamiento) :
        super (id, nombre, TipoRecurso.MILITAR) {
  }

  num getID() { return super.id; }
  String getNombre() { return super.getNombre(); }
  num getCosteUnitario() { return this._costeUnitario; }
  num getMaxUnidadesEnEntrenamiento() { return this.maxUnidadesEnEntrenamiento; }
}

class CivilConHonda extends UnidadMilitar {
  CivilConHonda (num vidaInicial, num danyoInflingido, num fuerza, num moral) :  super ( 1001, 'Civil con honda', vidaInicial, danyoInflingido, fuerza, moral, 1, 10) {}
}

class Soldado extends UnidadMilitar {
  Soldado (num vidaInicial, num danyoInflingido, num fuerza, num moral) : super ( 1002, 'Soldado', vidaInicial, danyoInflingido, fuerza, moral, 10, 10) {}
}

class Arquero extends UnidadMilitar {
  Arquero (num vidaInicial, num danyoInflingido, num fuerza, num moral) : super ( 1003, 'Arquero', vidaInicial, danyoInflingido, fuerza, moral, 5, 10) {}
}

class Lancero extends UnidadMilitar {
  Lancero (num vidaInicial, num danyoInflingido, num fuerza, num moral) : super ( 1002, 'Soldado', vidaInicial, danyoInflingido, fuerza, moral, 10, 10) {}
}
*/