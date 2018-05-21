pub mod cantera {
    pub use juego::almacen::almacen::Almacen;
    pub use juego::extractor::extractor::Extractor;
    pub use juego::productor::productor::Productor;
    pub use juego::edificio::edificio::{Edificio, TipoEdificio};
    pub use juego::punto::punto::Punto;
    pub use juego::recurso::recurso::{Recurso, TipoRecurso};
    pub use juego::dispatcher::dispatcher::Dispatcher;
    pub use juego::capital::capital::Capital;

    pub struct Cantera {
        //coste_construccion: i32,  de edificio
        //tiempo_construccion: i32, de edificio
        //posicion: Punto, de edificio
        cantidad_extraccion: i32,    //Parametros.Cantera_Productor_CantidadInicial;
        cantidad_maxima: i32,     //Parametros.Cantera_Productor_CantidadMaxima;
    
        id: i32,
        nombre: String,
        capital: Capital,
        disp: Dispatcher,
        canteros: Extractor,
        filon: Productor,
        almacen: Almacen, 

        edificio: Edificio,

        hay_envio_en_marcha: bool
    }

    impl Cantera {       
        
        pub fn new(id: i32, nombre:String, posicion: Punto, capital: Capital, dispatcher: Dispatcher) -> Cantera {
            /* Estos valores se obtendrán de clase Parametros */
            let cantidad_extraccion = 10; 
            let cantidad_maxima = 100000;
            let coste_construccion = 10;
            let tiempo_construccion = 10;

            let nombreEdificio: String = nombre.clone();

            let posicionEdificio: Punto = posicion.clone();
            let posicionProductor: Punto = posicion.clone();
            let posicionAlmacenCanteros: Punto = posicion.clone();
            let posicionAlmacen: Punto = posicion.clone();
            
            let edificio = Edificio::new(id,nombreEdificio,TipoEdificio::CANTERA_DE_PIEDRA,posicionEdificio,coste_construccion,tiempo_construccion);
            
            let PIEDRA: Recurso = Recurso::new(1, String::from("PIEDRA"), TipoRecurso::NATURAL); //Esto va como objeto constante en rescurso.rs            
            let recursoFilon: Recurso = PIEDRA.clone();
            let recursoAlmacenCanteros: Recurso = PIEDRA.clone();
            let recursoAlmacen: Recurso = PIEDRA.clone();
           
            let productor = Productor::new(posicionProductor,PIEDRA,0,10,1);
            
            let almacenCanteros = Almacen::new(recursoAlmacenCanteros,posicionAlmacenCanteros);
            let canteros = Extractor::new(productor,almacenCanteros,cantidad_extraccion);

            let filon = Productor::new(posicion,recursoFilon,999999,999999,1);
            
            let almacen = Almacen::new(recursoAlmacen,posicionAlmacen);

            //this._disp.addTareaRepetitiva(this, 'extrae', Parametros.Cantera_Cosecha_Tamanyo);
            //this.setStatus ('Sin envios actuales');

            Cantera {
                cantidad_extraccion: cantidad_extraccion,
                cantidad_maxima: cantidad_maxima,

                id: id,
                nombre: nombre,
                capital: capital,
                disp: dispatcher,
                canteros: canteros,
                filon: filon,
                almacen: almacen,

                edificio: edificio,

                hay_envio_en_marcha: false
            }
        }

        pub fn inicia(&self) {
            let child = thread::spawn(|| {
                self.extrae();
            });
            let _ = child.join();
        }   

        pub fn extrae(mut& self) {
            while self.estaActiva() { //En un futuro poner una guarda para cuando la mina se agote
                cantidad:i23 = self.canteros.get_cantidad();
                self.almacen.add_cantidad (cantidad);

                /* Si el almacen alcanza el tope enviar un transporte de piedra a palacio */
                if (self.almacen.get_cantidad() >= self.almacen.get_max_cantidad()) {
                    if (self.edificio.hay_envio_en_marcha() == false) {
                        self.edificio.set_envio_en_marcha(true);
                        self.envia_piedra_hacia_ciudad();
                    }
                }
                thread::sleep(Duration::from_secs(3)); //Parametrizar tiempo
            }
        }

        pub fn enviaPiedraHaciaCiudad(&self) {
            cantidad:i32 = self.almacen.resta_cantidad(self.almacen.get_cantidad());
            
            transporteDePiedra: Transporte = Transporte::new(self.almacen, capital.get_silo().get_almacen(), 
                self.almacen.get_recurso(), cantidad, self.edificio);

            this.edificio.setStatus ('inicio piedra...');
            transporteDePiedra.envia();
            this.edificio.setStatus ('Enviando piedra...');
        }

        pub fn estaActiva(&self) { this.filon.getStock() > 0 } //Parametrizar cero?

        pub fn get_piedra_actual(&self) -> i32 {
            self.almacen.get_cantidad()
        }

        pub fn get_max_almacen(&self) -> i32 {
            self.almacen.get_max_cantidad()
        }
    }
}