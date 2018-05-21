pub mod silo {
    pub struct Silo {
      capital: Capital
      almacenes: Vec<Almacen>
    }

    impl Silo {
      pub fn new(capital: Capital) {
        almacenes = Vec::new();  
        //tipo_recurso: Recurso, posicion: Punto
        let PIEDRA: Recurso = Recurso::new(1, String::from("PIEDRA"), TipoRecurso::NATURAL); //Esto va como objeto constante en rescurso.rs            
        almacen_piedra: Almacen = new Almacen(PIEDRA, capital.get_posicion()); 
        almacenes.push(almacen_piedra);

        Capital {
          capital: capital,
          almacenes: almacenes
        }
      }

      pub fn addAlmacen(&mut self, almacen: Almacen) {
          self.almacenes.push(almacen);
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