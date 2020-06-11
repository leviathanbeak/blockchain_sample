use std::sync::{Arc, Mutex};

extern crate actix_web;
extern crate serde_json;
extern crate blockchain;
mod services;

use actix_web::{middleware, App, HttpServer};
use services::{blockchain_api, mine_api, transactions_api};

#[derive(Debug, Clone)]
pub struct AppState {
    pub blockchain: Arc<Mutex<blockchain::Blockchain>>,
}

impl AppState {
    pub fn new(blockchain: Arc<Mutex<blockchain::Blockchain>>) -> Self {
        AppState {
            blockchain,
        }
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let address = "0.0.0.0:3000";

    let levicoin = blockchain::Blockchain::new();
    let levicoin = Arc::new(Mutex::new(levicoin));

    HttpServer::new(move || {
        let app_state = AppState::new(levicoin.clone());
        App::new()
            .data(app_state)
            .wrap(middleware::Logger::default())
            .service(blockchain_api::init_service())
            .service(transactions_api::init_service())
            .service(mine_api::init_service())
    })
    .bind(address)?
    .run()
    .await
}
