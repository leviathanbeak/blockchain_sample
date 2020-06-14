use crate::AppState;
use actix_web::{error, web, HttpResponse, Scope};
use blockchain::block::{Block, NonceNumber};
use blockchain::blockchain::{Blockchain, ConsensusOption};
use reqwest::Client;
use serde_json::json;

pub fn init_service() -> Scope {
    web::scope("/mine")
        .service(web::resource("").route(web::post().to(handle_post)))
        .service(web::resource("/receive").route(web::post().to(handle_receive)))
}

async fn handle_receive(
    block: web::Json<Block>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let block = block.0;

    match app_state.blockchain.lock() {
        Ok(mut blockchain) => {
            &blockchain.append_new_block(block);
            let res = json!({ "note": "New Block received!" });
            Ok(HttpResponse::Ok().json(res))
        }
        Err(e) => Err(error::ErrorInternalServerError(e.to_string())),
    }
}

async fn handle_post(app_state: web::Data<AppState>) -> Result<HttpResponse, error::Error> {
    let client = Client::new();

    let res = {
        match app_state.blockchain.lock() {
            Ok(mut blockchain) => {
                let previous_hash = blockchain.get_previous_hash();

                // ? prepare new block data
                let block_data = blockchain.format_pending_data();

                // ? find nonce with Proof of Work consensus
                let nonce: NonceNumber = Blockchain::consensus(ConsensusOption::ProofOfWork(
                    &previous_hash,
                    &block_data,
                ));

                // ? get hash with newfound nonce
                let block_hash = Blockchain::create_hash(&previous_hash, &block_data, nonce);

                // ? create new block in our chain
                let latest_mined_block =
                    blockchain.create_new_block(nonce, previous_hash, block_hash);

                // ? inform other nodes of the mined block
                futures::future::join_all(blockchain.network_nodes.iter().map(|node| {
                    let client = &client;
                    let block = &latest_mined_block;
                    async move {
                        let url = format!("http://localhost:{}/mine/receive", node.address);
                        client.post(&url).json(&block).send().await
                    }
                }))
                .await;

                // ? reward the miner that did the work with 10 coins
                let url = format!(
                    "http://localhost:{}/transaction/broadcast",
                    &blockchain.node.address
                );

                let json_res = json!({
                    "amount": 10,
                    "recipient": "The Miner",
                    "sender": "00",
                });

                Ok((url, json_res, latest_mined_block))
            }
            Err(e) => Err(e),
        }
    };

    match res {
        Ok((url, tx, block)) => {
            &client.post(&url).json(&tx).send().await;
            Ok(HttpResponse::Ok().json(&block))
        }
        Err(e) => Err(error::ErrorInternalServerError(e.to_string())),
    }
}
