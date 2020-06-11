# Blockchain Sample written in Rust

### Reusable Blockchain lib
```Rust
// create new chain
let mut blockchain = Blockchain::new();

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

### How to run it
```console
$ cd server
$ cargo build
$ cargo run
```

### How to use it
```
to get current chain
GET http://localhost:3000/blockchain

to post new Transaction
POST http://localhost:3000/transaction
body {
	"amount": 99,
	"sender": "elvis",
	"recipient": "elvis' friend"
}

to mine new Block
POST http://localhost:3000/mine
```