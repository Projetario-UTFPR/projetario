use std::sync::{Arc, Mutex};

pub mod coordenadores_de_projetos;
pub mod fabricas;

pub type TabelaThreadSafeEmMemoria<T> = Arc<Mutex<Vec<T>>>;
