pub mod transporte {
    pub struct Transporte {
        tiempo_recalculo: i32, //Parametros.Transporte_Tiempo_Recalculo_Ruta;
        velocidad: i32,        //Parametros.Transporte_Velocidad;

        posicion_actual: Punto,
        posicion_final: Punto
        ruta: &[Punto],

        almacen_origen: Almacen;
        almacen_destino: Almacen;
        recurso: Recurso;
        cantidad: i32;
        origen: Edificio;
    }

    impl Transporte {
        pub fn new(almacen_origen: Almacen, almacen_destino: Almacen, recurso: Recurso, cantidad: i32, origen: Edificio ) -> Transporte {
            Transporte {
                tiempo_recalculo: 3,
                velocidad: 1,
                posicion_actual: almacen_origen.get_posicion(),
                posicion_final: almacen_destino.get_posicion(),
                ruta: &[],
                almacen_origen = almacen_origen,
                almacen_destino = almacen_destino,
                recurso: recurso,
                cantidad: cantidad,
                origen: origen
            }
        }

        fn calcula_viaje(&mut self) {
            self.ruta = TomTom::calculaViaje (self.posicion_actual, self.posicion_final );
        }

        pub fn envia() {
            self.calcula_viaje();
            let child = thread::spawn(|| {
                for n in ruta {
                    self.posicion_actual = n;
                    thread::sleep(Duration::from_secs(self.tiempo_recalculo / self.velocidad);); //Modificar para valores reales (no enteros)
                }
                if ( Punto::sonIguales(self.posicion_actual, self.posicion_final)) ) {
                        //descarga mercancia                    
                        self.almacen_destino.add_cantidad(self.cantidad);
                        self.origen.setStatus(String::from("Envio finalizado"));
                        this.origen.set_envio_en_marcha(false);
                        en_ruta = false;
                    }
                }
            });
            let _ = child.join();
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