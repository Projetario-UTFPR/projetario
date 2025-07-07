use std::sync::atomic::{AtomicUsize, Ordering};

pub fn obtenha_esquema_unico_do_db() -> String {
    static ID: AtomicUsize = AtomicUsize::new(0);
    format!("test_schema_{}", ID.fetch_add(1, Ordering::SeqCst))
}
