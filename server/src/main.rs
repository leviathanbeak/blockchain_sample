use std::sync::{Arc, Mutex};

extern crate actix_web;
extern crate blockchain;
extern crate serde_json;

mod app_state;
mod args;
mod services;

use actix_web::{middleware, App, HttpServer};
use app_state::AppState;
use args::Args;
use blockchain::blockchain::Blockchain;
use blockchain::node::Node;
use reqwest::Client;
use services::{blockchain_api, mine_api, network_api, transactions_api};

// cargo watch -x "run -- -p 3000"
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let address = format!("0.0.0.0:{}", args.port);
    let levicoin = Blockchain::new(&args.port);
    let levicoin = Arc::new(Mutex::new(levicoin));

    let server = HttpServer::new(move || {
        let app_state = AppState::new(levicoin.clone());
        App::new()
            .data(app_state)
            .wrap(middleware::Logger::default())
            .service(blockchain_api::init_service())
            .service(transactions_api::init_service())
            .service(mine_api::init_service())
            .service(network_api::init_service())
    })
    .bind(address)?
    .run();

    match futures::future::join(server, run_broadcast(&args)).await {
        _ => Ok(())
    }
}

async fn run_broadcast(args: &Args) -> Result<(), reqwest::Error> {
    let port = args.port.clone();
    match args.master_address {
        Some(ref address) => {
            let url = format!("http://localhost:{}/network/register/broadcast", address);
            match Client::new()
                .post(&url)
                .json(&Node::new(&port))
                .send()
                .await
            {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        }
        _ => Ok(()),
    }
}
