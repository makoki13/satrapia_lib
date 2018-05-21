pub mod capital {
    pub use juego::punto::punto::Punto;
    pub use juego::edificios::silo::silo::Silo;

    #[derive(Clone)]
    pub struct Capital {
      posicion: Punto,
      silo: Silo
    }

    impl Capital {
      pub fn new(pos: Punto, silo: Silo) -> Capital {
        Capital {
          posicion: pos,
          silo: silo
        }
      }

      pub fn get_posicion(&self) -> Punto { self.posicion.clone() }

      pub fn get_silo(&self) -> Silo { self.silo.clone() }
    }
}

/*

class Capital extends Localidad {
  Palacio palacio;
  CentroDeInvestigacion centroDeInvestigacion;
  Silos silos;
  Cuartel cuartel;
  Mercado mercado;
  Embajada embajada;
  Taberna taberna;

  List<MinaDeOro> minasDeOro;
  List<Granja>granjas;
  List<Serreria> serrerias;
  List<Cantera> canteras;
  List<MinaDeHierro> minasDeHierro;

  num _id;
  String _nombre;
  Provincia _provincia;
  Punto _posicion;

  Capital(this._id, this._nombre, this._provincia, this._posicion) :  super(_id, _nombre, true, _provincia, 50, _posicion) {
    this.minasDeOro = new List<MinaDeOro>();
    this.granjas = new List<Granja>();
    this.serrerias = new List<Serreria>();
    this.canteras = new List<Cantera>();
    this.minasDeHierro = new List<MinaDeHierro>();
  }

  setPalacio (Palacio p) { this.palacio = p; }
  Palacio getPalacio () { return this.palacio; }

  setCentroDeInvestigacion (CentroDeInvestigacion c) { this.centroDeInvestigacion = c; }
  CentroDeInvestigacion getCentroDeInvestigacion () { return this.centroDeInvestigacion; }

  setSilos (Silos s) { this.silos = s; }
  Silos getSilos () { return this.silos; }

  setCuartel (Cuartel c) { this.cuartel = c; }
  Cuartel getCuartel () { return this.cuartel; }

  setMercado (Mercado m) { this.mercado = m; }
  getMercado () { return this.mercado; }

  setEmbajada (Embajada e) { this.embajada = e; }
  Embajada getEmbajada () { return this.embajada; }

  setTaberna (Taberna t) { this.taberna = t; }
  Taberna getTaberna () { return this.taberna; }

  addMinaDeOro(MinaDeOro m) { this.minasDeOro.add(m); }
  List<MinaDeOro> getMinasDeOro() { return this.minasDeOro; }

  addGranja(Granja g) { this.granjas.add(g); }
  List<Granja> getGranjas() { return this.granjas; }

  addSerreria(Serreria s) { this.serrerias.add(s); }
  List<Serreria> getSerrerias() { return this.serrerias; }

  addCantera(Cantera c) { this.canteras.add(c); }
  List<Cantera> getCanteras() { return this.canteras; }

  addMinaDeHierro(MinaDeHierro m) { this.minasDeHierro.add(m); }
  List<MinaDeHierro> getMinasDeHierro() { return this.minasDeHierro; }
}
*/