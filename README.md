# Aurora Indexer

This indexer watches for Aurora events (submit and submit_with_args transactions, will maybe support call and deploy_code in the future) and sends them to Redis stream `aurora_transaction`.

To run it, set `REDIS_URL` environment variable and `cargo run --release`
