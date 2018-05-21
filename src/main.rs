extern crate satrapia;

pub use satrapia::juego::almacen::almacen::Almacen;
pub use satrapia::juego::recurso::recurso::Recurso;
pub use satrapia::juego::recurso::recurso::TipoRecurso;
pub use satrapia::juego::punto::punto::Punto;
pub use satrapia::juego::militares::unidad_militar::unidad_militar::civil_con_honda::*;

fn main() {
    println!("Hola, Pollo!");

    let ORO:Recurso = Recurso::new(1, String::from("ORO"), TipoRecurso::NATURAL); //Esto va como objeto constante en rescurso.rs
    let posicion:Punto = Punto::new(100,100,100);

    let mut almacen = Almacen::new(ORO, posicion);
    almacen.add_cantidad(100);
    println!("almacen.get_cantidad {}", almacen.get_cantidad());

    let soldado = Civil_Con_Honda::new();
    println!("soldado._test {}", soldado._test())
}