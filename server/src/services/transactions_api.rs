use crate::AppState;
use actix_web::{error, web, HttpResponse, Scope};
use blockchain::transaction::Transaction;
use reqwest::Client;
use serde_json::json;
use serde::{Deserialize, Serialize};

pub fn init_service() -> Scope {
    web::scope("/transaction")
        .service(web::resource("").route(web::post().to(handle_post)))
        .service(web::resource("/broadcast").route(web::post().to(handle_broadcast)))
}

#[derive(Debug, Deserialize, Serialize)]
struct TxIntermediate {
    amount: u64,
    recipient: String,
    sender: String,
}

async fn handle_broadcast(
    transaction: web::Json<TxIntermediate>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let tx_im = transaction.0;
    let transaction = Transaction::new(tx_im.amount, tx_im.recipient, tx_im.sender);

    match app_state.blockchain.lock() {
        Ok(mut blockchain) => {            
            let o = &blockchain.append_new_transaction(transaction.clone());
            let client = Client::new();
            
            futures::future::join_all(blockchain.network_nodes.iter().map(|node| {
                let client = &client;
                let transaction = &transaction;
                async move {
                    let url = format!("http://localhost:{}/transaction", node.address);
                    client.post(&url).json(&transaction).send().await
                }
            }))
            .await;
     
            let res = json!({ "note": format!("it will be part of Block {}", o) });
            Ok(HttpResponse::Ok().json(res))
        }
        Err(e) => Err(error::ErrorInternalServerError(e.to_string()))
    }
}

async fn handle_post(
    transaction: web::Json<Transaction>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let transaction = transaction.0;
    match app_state.blockchain.lock() {
        Ok(mut blockchain) => {
            let o = &blockchain.append_new_transaction(transaction);
            let res = json!({ "note": format!("it will be part of Block {}", o) });
            Ok(HttpResponse::Ok().json(res))
        }
        Err(e) => Err(error::ErrorInternalServerError(e.to_string())),
    }
}
