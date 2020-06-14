use crate::AppState;
use actix_web::{error, web, HttpResponse, Scope};
use blockchain::blockchain::Blockchain;
use blockchain::node::Node;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub fn init_service() -> Scope {
    web::scope("/network/register")
        .service(web::resource("").route(web::post().to(handle_register)))
        .service(web::resource("/bulk").route(web::post().to(handle_bulk)))
        .service(web::resource("/broadcast").route(web::post().to(handle_broadcast)))
}

async fn handle_broadcast(
    node: web::Json<Node>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let new_node = node.0;
    let client = Client::new();

    let res = {
        match app_state.blockchain.lock() {
            Ok(mut blockchain) => {
                // broadcast to other nodes
                futures::future::join_all(blockchain.network_nodes.iter().map(|node| {
                    let client = &client;
                    let new_node = &new_node;
                    async move {
                        let url = format!("http://localhost:{}/network/register", node.address);
                        client.post(&url).json(&new_node).send().await
                    }
                }))
                .await;

                // append to current node
                blockchain.add_new_network_node(&new_node);

                // send other nodes to the new node
                let url = format!(
                    "http://localhost:{}/network/register/bulk",
                    &new_node.address
                );
                let nodes: Vec<Node> =
                    [&blockchain.network_nodes[..], &[blockchain.node.clone()]].concat();
                let json_res = json!({
                    "nodes": nodes,
                });

                Ok((url, json_res))
            }
            Err(er) => Err(er),
        }
    };

    match res {
        Ok((url, json_res)) => {
            &client.post(&url).json(&json_res).send().await;
            Ok(HttpResponse::Ok().json(new_node))
        }
        Err(e) => Err(error::ErrorInternalServerError(e.to_string())),
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct BulkNodes {
    nodes: Vec<Node>,
}

async fn handle_bulk(
    item: web::Json<BulkNodes>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    match app_state.blockchain.lock() {
        Ok(mut blockchain) => {
            for node in item.nodes.iter() {
                blockchain.add_new_network_node(&node);
            }

            let client = Client::new();
            let chains: Vec<Result<Blockchain, reqwest::Error>> =
                futures::future::join_all(blockchain.network_nodes.iter().map(|node| {
                    let client = &client;
                    async move {
                        let url = format!("http://localhost:{}/blockchain", node.address);
                        let res = client.get(&url).send().await?;
                        res.json::<Blockchain>().await
                    }
                }))
                .await;

            for result in chains {
                match result {
                    Ok(bc) => {
                        if bc.chain.len() > blockchain.chain.len() && bc.is_chain_valid() {
                            blockchain.chain = bc.chain;
                        }
                    }
                    _ => {}
                }
            }

            Ok(HttpResponse::Ok().json(&item.nodes))
        }
        Err(e) => Err(error::ErrorInternalServerError(e.to_string())),
    }
}

async fn handle_register(
    node: web::Json<Node>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, error::Error> {
    let node = node.0;

    match app_state.blockchain.lock() {
        Ok(mut blockchain) => {
            blockchain.add_new_network_node(&node);
            Ok(HttpResponse::Ok().json(node))
        }
        Err(e) => Err(error::ErrorInternalServerError(e.to_string())),
    }
}
