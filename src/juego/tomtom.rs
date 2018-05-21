pub mod tomtom {
    pub use juego::punto::punto::Punto;

    pub struct TomTom {
        ruta: Vec<Punto>
    }

    impl TomTom {
        pub fn calcula_viaje(origen: Punto, destino: Punto) -> Vec<Punto> {
            let mut ruta = Vec::new();

            let mut x_actual:i32 = origen.get_x();
            let mut y_actual:i32 = origen.get_y();

            let x_destino:i32 = destino.get_x();
            let y_destino:i32 = destino.get_y();

            let mut seguirCalculando:bool = true;
            while seguirCalculando == true {
                if x_actual > x_destino {
                    x_actual -= 1;
                } else {
                    if x_actual < x_destino {
                    x_actual += 1;
                    }
                }

                if y_actual > y_destino {
                    y_actual -= 1;
                } else {
                    if y_actual < y_destino {
                    y_actual += 1;
                    }
                }

                let nuevoPunto: Punto = Punto::new(x_actual, y_actual, 0);
                ruta.push(nuevoPunto);

                if (x_actual == x_destino) && (y_actual == y_destino) {
                    seguirCalculando = false;
                }
            }

            //Iterable listaReversed = ruta.reversed; ruta = listaReversed.toList(); //Darle la vuelta a la lista. Creo que no es necesario

            ruta
        }
    }
}