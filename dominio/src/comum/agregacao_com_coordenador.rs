use crate::identidade::entidades::professor::Professor;

pub trait AgregadoComCoordenador {
    fn obtenha_coordenador(&self) -> &Professor;
    fn obtenha_vice_coordenador(&self) -> Option<&Professor>;
}
