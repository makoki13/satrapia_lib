pub mod silo {
    pub use juego::almacen::almacen::Almacen;
    //pub use juego::capital::capital::Capital;
    pub use juego::punto::punto::Punto;
    pub use juego::recurso::recurso::{Recurso, TipoRecurso};

    #[derive(Clone)]
    pub struct Silo {      
      almacenes: Vec<Almacen>      
    }

    impl Silo {
      pub fn new(posicion: Punto) -> Silo {
        let mut almacenes = Vec::new();  
        //tipo_recurso: Recurso, posicion: Punto
        let PIEDRA: Recurso = Recurso::new(1, String::from("PIEDRA"), TipoRecurso::NATURAL); //Esto va como objeto constante en rescurso.rs            
        let almacen_piedra: Almacen = Almacen::new(PIEDRA, posicion); 
        almacenes.push(almacen_piedra);

        Silo {          
          almacenes: almacenes          
        }
      }

      pub fn addAlmacen(&mut self, almacen: Almacen) {
          self.almacenes.push(almacen);
      }

      pub fn get_almacen_de_piedra(&self) -> Almacen {
        self.almacenes[0].clone()
      }
    }
}

/*
class Silos extends Edificio {    
  List<Almacen> getLista() {return this.almacenes;}

  Almacen getAlmacenComida() {
    Almacen a = null;
    this.almacenes.forEach( (almacen) {
        print("iteracion almacen comida: "+almacen.toString());
        if (almacen.getTipoRecurso().id == COMIDA.id) {
          a = almacen;
        }
    });
    return a;
  }

  Almacen getAlmacenMadera() {
    Almacen al = null;
    this.almacenes.forEach( (almacen) {
      if (almacen.getTipoRecurso() == MADERA) { al = almacen; return; }
    });
    return al;
  }

  Almacen getAlmacenPiedra() {
    Almacen al = null;
    this.almacenes.forEach( (almacen) {
      if (almacen.getTipoRecurso() == PIEDRA) { al = almacen; return; }
    });
    return al;
  }

  Almacen getAlmacenHierro() {
    Almacen al = null;
    this.almacenes.forEach( (almacen) {
      if (almacen.getTipoRecurso() == HIERRO) { al = almacen; return; }
    });
    return al;
  }
}
*/