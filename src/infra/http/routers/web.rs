use super::RouterTrait;

pub struct WebRouter;

impl RouterTrait for WebRouter {
    fn register(cfg: &mut actix_web::web::ServiceConfig) {}
}
