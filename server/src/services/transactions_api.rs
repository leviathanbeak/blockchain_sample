use crate::AppState;
use actix_web::{error, web, HttpResponse, Resource};
use blockchain::transaction::Transaction;
use bytes::BytesMut;
use futures::StreamExt;
use serde_json::json;

pub fn init_service() -> Resource {
    web::resource("/transaction").route(web::post().to(handle_post))
}

async fn handle_post(
    mut payload: web::Payload,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let mut body = BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        body.extend_from_slice(&chunk);
    }

    match serde_json::from_slice::<Transaction>(&body) {
        Ok(res) => match app_state.blockchain.lock() {
            Ok(mut blockchain) => {
                let o = &blockchain.create_new_transaction(res);
                let res = json!({ "note": format!("it will be part of Block {}", o) });
                Ok(HttpResponse::Ok().json(res))
            }
            Err(e) => Err(error::ErrorInternalServerError(e.to_string())),
        },
        Err(e) => Err(error::ErrorBadRequest(e)),
    }
}
