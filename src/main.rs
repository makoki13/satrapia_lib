extern crate satrapia;

pub use satrapia::juego::almacen::almacen::Almacen;
pub use satrapia::juego::recurso::recurso::Recurso;
pub use satrapia::juego::recurso::recurso::TipoRecurso;
pub use satrapia::juego::punto::punto::Punto;
pub use satrapia::juego::militares::unidad_militar::unidad_militar::civil_con_honda::*;

pub use satrapia::juego::edificios::cantera::cantera::Cantera;
pub use satrapia::juego::capital::capital::Capital;
pub use satrapia::juego::edificios::silo::silo::Silo;

fn main() {
    println!("Hola, Pollo!");

    //let ORO:Recurso = Recurso::new(1, String::from("ORO"), TipoRecurso::NATURAL); //Esto va como objeto constante en rescurso.rs
    /*
    let mut almacen = Almacen::new(ORO, posicion);
    almacen.add_cantidad(100);
    println!("almacen.get_cantidad {}", almacen.get_cantidad());
    */

    let posicion_capital:Punto = Punto::new(0,0,0);

    let silo = Silo::new(posicion_capital.clone());    
    let mut capital = Capital::new(posicion_capital, silo);
    
    let posicion:Punto = Punto::new(10,10,10);
    let mut cantera = Cantera::new(1, String::from("Canterilla"), posicion, &mut capital);

    cantera.extrae();

    loop  {}

    //let soldado = Civil_Con_Honda::new();
    //println!("soldado._test {}", soldado._test())
}