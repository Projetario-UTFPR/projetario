#[macro_export]
macro_rules! instanciar_app {
    ($inertia_instancia:expr, $db_guard_instancia:expr) => {
        actix_web::test::init_service(
            projetario::server::get_server()
                .app_data($inertia_instancia)
                .app_data($db_guard_instancia)
                .app_data(projetario::infra::tema::GerenteDeTema::new(true)),
        )
        .await
    };
}
