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

        edificio: Edificio
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

                edificio: edificio
            }
        }
    }
}