use crate::AppState;
use actix_web::{error, web, HttpResponse, Resource};
use blockchain::block::NonceNumber;
use blockchain::blockchain::{Blockchain, ConsensusOption};
use blockchain::transaction;
use serde_json::json;

pub fn init_service() -> Resource {
    web::resource("/mine").route(web::post().to(handle_post))
}

async fn handle_post(app_state: web::Data<AppState>) -> Result<HttpResponse, error::Error> {
    match app_state.blockchain.lock() {
        Ok(mut blockchain) => {
            let blockchain = &mut *blockchain;
            let previous_hash = blockchain.get_previous_hash();

            // ? prepare new block data
            let block_data = blockchain.format_pending_data();

            // ? find nonce with Proof of Work consensus
            let nonce: NonceNumber =
                Blockchain::consensus(ConsensusOption::ProofOfWork(&previous_hash, &block_data));

            // ? get hash with newfound nonce
            let block_hash = Blockchain::create_hash(&previous_hash, &block_data, nonce);

            // ? reward the miner that did the work with 10 coins
            blockchain.create_new_transaction(transaction::Transaction::new(
                10,
                "da Miner".to_owned(),
                "da Chain".to_owned(),
            ));

            // ? create new block in our chain
            let latest_mined_block = blockchain.create_new_block(nonce, previous_hash, block_hash);

            let res = json!({
                "block": latest_mined_block,
            });

            Ok(HttpResponse::Ok().json(res))
        }
        Err(e) => Err(error::ErrorInternalServerError(e.to_string())),
    }
}
