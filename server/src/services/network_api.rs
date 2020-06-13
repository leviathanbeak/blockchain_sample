use crate::AppState;
use actix_web::{error, web, HttpResponse, Scope};
use blockchain::node::Node;
use bytes::BytesMut;
use futures::StreamExt;

pub fn init_service() -> Scope {
    web::scope("/network")
        .service(web::resource("/register").route(web::post().to(handle_register)))
        .service(web::resource("/register/bulk").route(web::post().to(handle_bulk)))
}

async fn handle_bulk(
    mut payload: web::Payload,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let mut body = BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        body.extend_from_slice(&chunk);
    }

    match serde_json::from_slice::<Vec<Node>>(&body) {
        Ok(res) => match app_state.blockchain.lock() {
            Ok(mut blockchain) => {
                for node in res.iter() {
                    blockchain.add_new_network_node(&node);
                }
                Ok(HttpResponse::Ok().json(res))
            }
            Err(e) => Err(error::ErrorInternalServerError(e.to_string())),
        },
        Err(e) => Err(error::ErrorBadRequest(e)),
    }
}

async fn handle_register(
    mut payload: web::Payload,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let mut body = BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        body.extend_from_slice(&chunk);
    }

    match serde_json::from_slice::<Node>(&body) {
        Ok(res) => match app_state.blockchain.lock() {
            Ok(mut blockchain) => {
                println!(" {:?}", res);
                blockchain.add_new_network_node(&res);
                Ok(HttpResponse::Ok().json(res))
            }
            Err(e) => Err(error::ErrorInternalServerError(e.to_string())),
        },
        Err(e) => Err(error::ErrorBadRequest(e)),
    }
}
