use crate::identidade::entidades::usuario::UsuarioModelo;

pub trait IntoUsuarioModelo {
    fn into_usuario_modelo(self) -> UsuarioModelo;
}
