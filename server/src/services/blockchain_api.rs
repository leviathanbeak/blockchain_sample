use actix_web::{web, error, HttpResponse, Resource};
use crate::AppState;

pub fn init_service() -> Resource {
    web::resource("/blockchain")
        .route(web::get().to(handle_get))
}

async fn handle_get(app_state: web::Data<AppState>) -> Result<HttpResponse, error::Error> {
    match app_state.blockchain.lock() {
        Ok(guard) => Ok(HttpResponse::Ok().json(&*guard)),
        Err(e) => Err(error::ErrorInternalServerError(e.to_string()))
    }
}