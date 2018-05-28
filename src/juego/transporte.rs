pub mod transporte {
    pub use juego::punto::punto::Punto;
    pub use juego::almacen::almacen::Almacen;
    pub use juego::recurso::recurso::Recurso;
    pub use juego::edificio::edificio::Edificio;
    pub use juego::tomtom::tomtom::TomTom;

    use std::thread;
    use std::time::Duration;
    //use std::sync::mpsc;
    
    pub struct Transporte {
        tiempo_recalculo: i32,
        velocidad: i32,
        posicion_actual: Punto,
        posicion_final: Punto,        
        recurso: Recurso,
        cantidad: i32,        
        ruta: Vec<Punto>,
    }

    impl Transporte {
        pub fn new(recurso: Recurso, cantidad: i32) -> Transporte {
            let ruta = Vec::new(); 
            let pos_actual = Punto::new(10,10,10);
            let pos_final = Punto::new(0,0,0);
            Transporte {
                tiempo_recalculo: 3,
                velocidad: 1,
                posicion_actual: pos_actual,
                posicion_final: pos_final,                
                recurso: recurso,
                cantidad: cantidad,                
                ruta: ruta
            }
        }

        fn calcula_viaje(&mut self) {
            self.ruta = TomTom::calcula_viaje (self.posicion_actual.clone(), self.posicion_final.clone() );
        }

        pub fn envia(&mut self) {
            //let tiempo:u64 = self.tiempo_recalculo;
            self.calcula_viaje();
            let rutaCalculada = self.ruta.clone();
            let posicion_final = self.posicion_final.clone();
            let mut posicion_actual = self.posicion_actual.clone();

            //let (tx, rx) = mpsc::channel();

            //let child = thread::spawn(move || {
                
                let tiempo:u64 = 1; //PENDIENTE
                                
                //
                //self.ruta = TomTom::calcula_viaje (self.posicion_actual.clone(), self.posicion_final.clone() );
                                
                for mut n in rutaCalculada {
                    posicion_actual = n;
                    println!("Pos {} {}", posicion_actual.get_x(), posicion_actual.get_y());
                    //pos_act = posicion_actual;
                    //tx.send(posicion_actual).unwrap();
                    thread::sleep(Duration::from_secs(tiempo)); //Modificar para valores reales (no enteros)
                }

                /*            
                if Punto::son_iguales(pos_act, posicion_final) {
                    //descarga mercancia                    
                    
                    /*
                    self.almacen_de_destino.add_cantidad(self.cantidad);                    
                    self.origen.setStatus(String::from("Envio finalizado"));
                    self.origen.set_envio_en_marcha(false);
                    */
                } 
                */
                
            //});

            
            //let posicion = rx.recv().unwrap();
            //println!("Pos {} {}", posicion.get_x(), posicion.get_y());
            
            
            //let res = child.join();
        }
    }
}
/*
class Transporte {
  
  myCallBack(String funcion, [List<dynamic>parametros]) {
    switch(funcion) {
      case 'envia': this.envia(); return;
      default:
    }
  }

  calculaViaje() {
    this.ruta = TomTom.calculaViaje (this.posicionActual, this.posicionFinal );
  }

  envia() {
    this.modificaPosicionActual();
    // console.log ('Pos: ' + this.posicionActual.getX() + ',' + this.posicionActual.getY() );
    if ( (this.ruta.length == 0) && (Punto.sonIguales(this.posicionActual, this.posicionFinal)) ) {
      // console.log ('descarga en palacio');
      this._almacenDestino.addCantidad(this._cantidad);
      this._origen.setStatus('Envio finalizado');
      this._origen.hayEnvioEnMarcha = false;
      return -1; // suicidio
    }
  }

  modificaPosicionActual() {
    this.posicionActual = this.ruta.removeAt(0);
  }
}
*/