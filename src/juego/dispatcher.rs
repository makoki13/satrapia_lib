/** TERMINADO */
pub mod dispatcher {
    #[derive(Clone)]
    pub struct Dispatcher {
        id: i32
    }

    impl Dispatcher {
         pub fn new() -> Dispatcher {
             Dispatcher {
                 id: 0
             }
         }
    }
}