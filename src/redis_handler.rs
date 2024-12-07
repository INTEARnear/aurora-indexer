use async_trait::async_trait;
use inevents_redis::RedisEventStream;
use inindexer::near_indexer_primitives::types::BlockHeight;
use intear_events::events::aurora::transaction::AuroraTransactionEvent;
use redis::aio::ConnectionManager;

use crate::AuroraEventHandler;

pub struct PushToRedisStream {
    transactions_stream: RedisEventStream<AuroraTransactionEvent>,
    max_stream_size: usize,
}

impl PushToRedisStream {
    pub async fn new(connection: ConnectionManager, max_stream_size: usize) -> Self {
        Self {
            transactions_stream: RedisEventStream::new(
                connection.clone(),
                AuroraTransactionEvent::ID,
            ),
            max_stream_size,
        }
    }
}

#[async_trait]
impl AuroraEventHandler for PushToRedisStream {
    async fn handle_transaction(&mut self, event: AuroraTransactionEvent) {
        self.transactions_stream.add_event(event);
    }

    async fn flush_events(&mut self, block_height: BlockHeight) {
        self.transactions_stream
            .flush_events(block_height, self.max_stream_size)
            .await
            .expect("Failed to flush transactions stream");
    }
}
