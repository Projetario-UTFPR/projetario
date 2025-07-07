use actix_web::web::Data;
use inertia_rust::Inertia;
use rstest::fixture;

#[fixture]
pub async fn inertia() -> Data<Inertia> {
    let vite = config::vite::get_vite()
        .await
        .expect("Failed to initialize vite in test fixture.");

    let inertia = config::inertia::get_inertia(vite)
        .await
        .expect("Failed to initialize inertia in test fixture.");

    Data::new(inertia)
}
