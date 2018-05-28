pub mod cantera {
    pub use juego::almacen::almacen::Almacen;
    pub use juego::extractor::extractor::Extractor;
    pub use juego::productor::productor::Productor;
    pub use juego::edificio::edificio::{Edificio, TipoEdificio};
    pub use juego::punto::punto::Punto;
    pub use juego::recurso::recurso::{Recurso, TipoRecurso};    
    pub use juego::capital::capital::Capital;
    pub use juego::transporte::transporte::Transporte;

    use std::thread;
    use std::time::Duration;
    use std::sync::mpsc;
    
    //#[derive(Clone)]
    pub struct Cantera<'a> {
        //coste_construccion: i32,  de edificio
        //tiempo_construccion: i32, de edificio
        //posicion: Punto, de edificio
        cantidad_extraccion: i32,    //Parametros.Cantera_Productor_CantidadInicial;
        cantidad_maxima: i32,     //Parametros.Cantera_Productor_CantidadMaxima;
    
        id: i32,
        nombre: String,
        capital: &'a mut Capital,        
        canteros: Extractor,
        filon: Productor,
        almacen: Almacen, 

        edificio: Edificio,

        hay_envio_en_marcha: bool
    }

    impl<'a> Cantera<'a> {
        
        pub fn new(id: i32, nombre:String, posicion: Punto, capital: &'a mut Capital) -> Cantera<'a> {
            /* Estos valores se obtendrán de clase Parametros */
            let cantidad_extraccion = 10; 
            let cantidad_maxima = 100000;
            let coste_construccion = 10;
            let tiempo_construccion = 10;

            let nombreEdificio: String = nombre.clone();

            let posicionEdificio: Punto = posicion.clone();
            //let posicionProductor: Punto = posicion.clone();
            let posicionAlmacenCanteros: Punto = posicion.clone();
            let posicionAlmacen: Punto = posicion.clone();
            
            let edificio = Edificio::new(id,nombreEdificio,TipoEdificio::CANTERA_DE_PIEDRA,posicionEdificio,coste_construccion,tiempo_construccion);
            
            let PIEDRA: Recurso = Recurso::new(1, String::from("PIEDRA"), TipoRecurso::NATURAL); //Esto va como objeto constante en rescurso.rs            
            let recursoFilon: Recurso = PIEDRA.clone();
            let recursoAlmacenCanteros: Recurso = PIEDRA.clone();
            let recursoAlmacen: Recurso = PIEDRA.clone();
           
            //let productor = Productor::new(posicionProductor,PIEDRA,0,10,1);
            
            let almacenCanteros = Almacen::new(recursoAlmacenCanteros,posicionAlmacenCanteros);
            
            let filon = Productor::new(posicion,recursoFilon,999999,999999,1);
            let canteros = Extractor::new(filon.clone(),almacenCanteros,cantidad_extraccion);
                        
            let almacen = Almacen::new(recursoAlmacen,posicionAlmacen);

            //this._disp.addTareaRepetitiva(this, 'extrae', Parametros.Cantera_Cosecha_Tamanyo);
            //this.setStatus ('Sin envios actuales');

            Cantera {
                cantidad_extraccion: cantidad_extraccion,
                cantidad_maxima: cantidad_maxima,

                id: id,
                nombre: nombre,
                capital: capital,                
                canteros: canteros,
                filon: filon,
                almacen: almacen,

                edificio: edificio,

                hay_envio_en_marcha: false
            }
        }

        pub fn inicia(&mut self) {
            self.extrae();            
        }   

        pub fn extrae(&mut self) {
            while self.esta_activa() {
                let cantidad:i32 = self.canteros.get_cantidad();
                println!("Cantidad extraida: {}", cantidad);
                self.almacen.add_cantidad (cantidad);

                println!("Cantidad: {}", self.almacen.get_cantidad());

                //let (tx, rx) = mpsc::channel();
                /* Si el almacen alcanza el tope enviar un transporte de piedra a palacio */
                if self.almacen.get_cantidad() >= self.almacen.get_max_cantidad() {
                    println!("{}",self.edificio.hay_envio_en_marcha());
                    if self.edificio.hay_envio_en_marcha() == false {                        
                        self.edificio.set_envio_en_marcha(true);     
                        println!("Enviando...");  
                        println!("{}",self.edificio.hay_envio_en_marcha());                 

                        let cantidadAlmacen = self.almacen.get_cantidad();             
                        let cantidad:i32 = self.almacen.resta_cantidad(cantidadAlmacen);
                        let recurso = self.almacen.get_recurso();

                        let almacen_origen = &mut self.almacen;
                        let almacen_destino = &mut self.capital.get_silo().get_almacen_de_piedra();

                        //let edificio = self.edificio;                        
                        thread::spawn(move || {
                            Cantera::envia_piedra_hacia_ciudad(cantidad, recurso); //Debe de ser un thread...                                                    
                            //tx.send(mensaje).unwrap();
                        });
                        self.edificio.set_envio_en_marcha(false);     
                    }
                    else {
                        //let posicion = rx.recv().unwrap();
                        //println!("{}", posicion);
                        println!("Esperando final de envio");
                    }                    
                }                                
                thread::sleep(Duration::from_secs(3)); //Parametrizar tiempo
            }            
        }


        pub fn envia_piedra_hacia_ciudad(cantidad:i32, recurso:Recurso) -> String {                        
            let mut transporteDePiedra: Transporte = Transporte::new(recurso, cantidad);            
            transporteDePiedra.envia();            
            String::from("Hola")
        }

        pub fn esta_activa(&self) -> bool { self.filon.getStock() > 0 } //Parametrizar cero?

        pub fn get_piedra_actual(&self) -> i32 {
            self.almacen.get_cantidad()
        }

        pub fn get_max_almacen(&self) -> i32 {
            self.almacen.get_max_cantidad()
        }
    }
}