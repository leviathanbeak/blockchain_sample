# Blockchain Sample written in Rust

```txt
This is a toy example of Blockchain Data structure,
complete with peer 2 peer newtwork written in Rust with Actix-Web
it implements:
```
- [x] Creating, Sending & Storing Transactions (albeit no security)
- [x] Consensus Algorithm: Proof of Work
- [x] Peer to Peer network (broadcast & consensus with longest chain wins)
- [x] Performing Chain validity with Hashes (Sha256)
- [x] Mining Blocks

### Reusable Blockchain lib
```Rust
// Blockchain Struct
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
    pub node: Node,
    pub network_nodes: Vec<Node>,
}

// create new chain with address of your node
let mut blockchain = Blockchain::new(address: &NodeAddress);

// append new tx
blockchain.create_new_transaction(Transaction::new(100, "me".to_owned(), "you".to_owned()));

// get previous block hash
let previous_hash = blockchain.get_previous_hash();

// prepare new block data
let block_data = blockchain.format_pending_data();

// find nonce with Proof of Work consensus
let nonce: NonceNumber = Blockchain::consensus(ConsensusOption::ProofOfWork(
    &previous_hash,
    &block_data,
));

// get hash with newfound nonce
let block_hash = Blockchain::create_hash(&previous_hash, &block_data, nonce);

// reward the miner that did the work with 10 coins
blockchain.create_new_transaction(Transaction::new(10, "da Miner".to_owned(), "da Chain".to_owned()));

// create new block in our chain
let latest_mined_block = blockchain.create_new_block(nonce, previous_hash, block_hash);

// check is chain valid
blockchain.is_chain_valid()

// add new peer node
blockchain.add_new_network_node(node)
```

## How to use the Server example
### Currently only available for local testing
```console
$ cd server
$ cargo build
$ cargo run -- --port 3000 // start first node instance at port 3000
$ cargo run -- --port 3001 -m 3000 // start another instance, that connects to the node at 3000
// make some transactions, mine blocks, join new nodes
$ cargo run -- --port 3002
```

### API Endpoints
```
- to get current chain
GET http://localhost:3000/blockchain

- to post new Transaction
POST http://localhost:3001/transaction/broadcast
body {
	"amount": 99,
	"sender": "elvis",
	"recipient": "elvis' friend"
}

- to mine new Block
POST http://localhost:3000/mine

- to register new node
POST http://localhost:3000/network/register
body {
    "address": "3001"	// local port
}

- POST register all other nodes
http://localhost:3000/network/register/bulk
body {
    [
        {
            "address": "3001"
        },
        {
            "address": "3002"
        },
        {
            "address": "3003"
        }
    ]
}
```

### API ready
```js
// GET - /blockchain
// Response 200 OK
{
    "chain": [
        {
            "index": 0,
            "timestamp": {
                "secs_since_epoch": 1591877615,
                "nanos_since_epoch": 447657200
            },
            "transactions": [],
            "nonce": 0,
            "hash": "myhash",
            "previous_block_hash": "prehash"
        },
        {
            "index": 1,
            "timestamp": {
                "secs_since_epoch": 1591877627,
                "nanos_since_epoch": 102672800
            },
            "transactions": [
                {
                    "amount": 99,
                    "recipient": "myfriend",
                    "sender": "elvis"
                },
                {
                    "amount": 10,
                    "recipient": "da Miner",
                    "sender": "da Chain"
                }
            ],
            "nonce": 113989,
            "hash": "0000898840ac311e53bf96c340dc556a07155dc066cdb66467393eb76d336a45",
            "previous_block_hash": "myhash"
        }
    ],
    "pending_transactions": []
}

// POST - /transaction
// Response 200 OK
{
    "note": "it will be part of Block 1"
}

// POST - /mine
// Response 200 OK
{
    "block": {
        "hash": "0000898840ac311e53bf96c340dc556a07155dc066cdb66467393eb76d336a45",
        "index": 1,
        "nonce": 113989,
        "previous_block_hash": "myhash",
        "timestamp": {
            "nanos_since_epoch": 102672800,
            "secs_since_epoch": 1591877627
        },
        "transactions": [
            {
                "amount": 99,
                "recipient": "samba",
                "sender": "elvis"
            },
            {
                "amount": 10,
                "recipient": "da Miner",
                "sender": "da Chain"
            }
        ]
    }
}
```