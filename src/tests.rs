use async_trait::async_trait;
use aurora_indexer::{AuroraEventHandler, AuroraIndexer};
use inindexer::{
    near_indexer_primitives::types::BlockHeight, neardata::NeardataProvider, run_indexer,
    BlockRange, IndexerOptions, PreprocessTransactionsSettings,
};

use intear_events::events::aurora::transaction::AuroraTransactionEvent;

struct TestHandler {
    transactions: Vec<AuroraTransactionEvent>,
}

#[async_trait]
impl AuroraEventHandler for TestHandler {
    async fn handle_transaction(&mut self, event: AuroraTransactionEvent) {
        self.transactions.push(event);
    }

    async fn flush_events(&mut self, _block_height: BlockHeight) {}
}

#[tokio::test]
async fn detects_submit() {
    let handler = TestHandler {
        transactions: Vec::new(),
    };

    let mut indexer = AuroraIndexer(handler);

    run_indexer(
        &mut indexer,
        NeardataProvider::mainnet(),
        IndexerOptions {
            preprocess_transactions: Some(PreprocessTransactionsSettings {
                prefetch_blocks: 0,
                postfetch_blocks: 0,
            }),
            ..IndexerOptions::default_with_range(BlockRange::Range {
                start_inclusive: 134295233,
                end_exclusive: Some(134295235),
            })
        },
    )
    .await
    .unwrap();

    assert_eq!(
        format!("{:?}", indexer.0.transactions),
        "[AuroraTransactionEvent { block_height: 134295234, block_timestamp_nanosec: 1733415730530401689, transaction_id: BeBEBnwmtLJEoCtze8L7BhpMbtnKQbJde3Ux8cBp3izC, receipt_id: 8gXqAgWPqU5T6hqzsmo2m3jxHnrH9v4yQ7PmVVJA7zSL, chain_id: Some(1313161554), aurora_tx_hash: \"0x661b…8805\", from: Address(0x0143ecf011c1996b3446f15eb9da409d216d491b), to: Some(Address(0xef76a77cf5eae4fe5031eb7f0f95bb2788c72f19)), value: Wei(0), input: [65, 38, 88, 229, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 92, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 39, 0, 139, 1, 195, 2, 29, 2, 39, 2, 49, 2, 59, 2, 75, 2, 91, 196, 44, 48, 172, 108, 193, 95, 172, 155, 217, 56, 97, 139, 202, 161, 161, 250, 232, 80, 29, 73, 136, 168, 150, 177, 34, 114, 24, 228, 166, 134, 253, 229, 234, 189, 202, 189, 145, 87, 31, 218, 37, 133, 67, 15, 239, 50, 122, 216, 238, 68, 175, 143, 31, 152, 154, 42, 145, 163, 210, 255, 121, 213, 191, 244, 142, 28, 1, 183, 34, 86, 13, 111, 253, 252, 233, 252, 136, 53, 135, 92, 233, 240, 182, 175, 179, 97, 53, 181, 221, 191, 17, 112, 92, 235, 101, 230, 52, 169, 220, 0, 149, 0, 170, 1, 2, 1, 88, 1, 174, 20, 243, 222, 157, 195, 143, 98, 96, 129, 121, 196, 95, 232, 148, 58, 12, 163, 75, 169, 206, 252, 87, 101, 29, 188, 97, 168, 163, 143, 159, 223, 191, 227, 120, 228, 221, 212, 233, 94, 132, 159, 220, 163, 67, 21, 241, 239, 73, 57, 35, 135, 221, 20, 63, 69, 120, 8, 58, 155, 211, 62, 148, 218, 37, 133, 67, 15, 239, 50, 122, 216, 238, 68, 175, 143, 31, 152, 154, 42, 145, 163, 210, 73, 136, 168, 150, 177, 34, 114, 24, 228, 166, 134, 253, 229, 234, 189, 202, 189, 145, 87, 31, 0, 3, 4, 18, 18, 6, 6, 85, 24, 19, 26, 154, 244, 156, 68, 118, 223, 60, 52, 47, 13, 39, 236, 216, 220, 106, 188, 80, 163, 67, 21, 241, 239, 73, 57, 35, 135, 221, 20, 63, 69, 120, 8, 58, 155, 211, 62, 148, 218, 37, 133, 67, 15, 239, 50, 122, 216, 238, 68, 175, 143, 31, 152, 154, 42, 145, 163, 210, 255, 121, 213, 191, 244, 142, 28, 1, 183, 34, 86, 13, 111, 253, 252, 233, 252, 136, 53, 135, 0, 1, 2, 18, 18, 85, 159, 49, 155, 175, 219, 208, 31, 152, 128, 254, 233, 132, 233, 100, 162, 70, 221, 43, 133, 207, 143, 228, 79, 92, 206, 2, 213, 190, 68, 227, 68, 107, 188, 46, 129, 50, 149, 141, 34, 184, 92, 233, 240, 182, 175, 179, 97, 53, 181, 221, 191, 17, 112, 92, 235, 101, 230, 52, 169, 220, 255, 121, 213, 191, 244, 142, 28, 1, 183, 34, 86, 13, 111, 253, 252, 233, 252, 136, 53, 135, 0, 1, 2, 18, 18, 20, 169, 237, 237, 62, 51, 155, 156, 217, 43, 182, 222, 245, 197, 55, 157, 103, 129, 49, 255, 144, 1, 205, 1, 225, 1, 245, 1, 245, 2, 9, 161, 177, 116, 46, 156, 50, 199, 202, 169, 114, 109, 130, 4, 189, 87, 21, 227, 65, 152, 97, 60, 46, 49, 206, 220, 34, 248, 83, 33, 196, 118, 178, 73, 156, 48, 187, 222, 189, 201, 177, 110, 182, 98, 110, 89, 250, 50, 238, 156, 47, 171, 161, 192, 134, 182, 91, 45, 76, 91, 215, 44, 180, 94, 219, 69, 23, 213, 148, 122, 253, 227, 190, 171, 249, 90, 88, 37, 6, 133, 139, 0, 0, 1, 225, 1, 245, 1, 245, 0, 0, 0, 30, 0, 4, 0, 4, 0, 4, 0, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 105, 95, 34, 255, 223, 3, 103, 209, 152, 0, 0, 0, 0, 0, 0, 0, 5, 136, 157, 127, 126, 143, 227, 154, 112, 128, 0, 0, 0, 0, 0], status: Revert([8, 195, 121, 160, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 110, 111, 112, 58, 32, 97, 109, 111, 117, 110, 116, 32, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) }, AuroraTransactionEvent { block_height: 134295234, block_timestamp_nanosec: 1733415730530401689, transaction_id: 5CEUipSY5eaCcaQh7uSRn6NpR3PtSTh2YEx2f2iyRDpP, receipt_id: 6DuxLJNCb7UdjDd7xFDm3h4Kjno6ts8DZMug75byZMqK, chain_id: Some(1313161554), aurora_tx_hash: \"0x503c…f6b6\", from: Address(0x07aad693d3b7862b58be68d814802e189d66bf21), to: Some(Address(0xef76a77cf5eae4fe5031eb7f0f95bb2788c72f19)), value: Wei(0), input: [65, 38, 88, 229, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 39, 0, 99, 1, 228, 2, 18, 2, 24, 2, 30, 2, 36, 2, 52, 2, 68, 196, 44, 48, 172, 108, 193, 95, 172, 155, 217, 56, 97, 139, 202, 161, 161, 250, 232, 80, 29, 139, 236, 71, 134, 90, 222, 59, 23, 42, 146, 141, 248, 249, 144, 188, 127, 42, 59, 159, 121, 70, 33, 141, 25, 98, 243, 157, 150, 185, 41, 90, 134, 208, 24, 73, 243, 107, 245, 42, 62, 0, 105, 0, 126, 1, 49, 20, 30, 14, 129, 47, 188, 211, 235, 117, 216, 86, 42, 214, 243, 16, 237, 148, 210, 88, 208, 8, 178, 99, 116, 91, 79, 126, 15, 62, 151, 65, 220, 252, 211, 33, 72, 175, 221, 8, 95, 251, 129, 232, 78, 84, 24, 67, 43, 238, 235, 84, 59, 188, 223, 159, 165, 79, 215, 234, 58, 166, 91, 70, 33, 141, 25, 98, 243, 157, 150, 185, 41, 90, 134, 208, 24, 73, 243, 107, 245, 42, 62, 139, 236, 71, 134, 90, 222, 59, 23, 42, 146, 141, 248, 249, 144, 188, 127, 42, 59, 159, 121, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 8, 83, 160, 210, 49, 60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 141, 21, 225, 118, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 224, 182, 179, 167, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 224, 182, 179, 167, 100, 0, 0, 232, 78, 84, 24, 67, 43, 238, 235, 84, 59, 188, 223, 159, 165, 79, 215, 234, 58, 166, 91, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 73, 178, 15, 220, 12, 203, 74, 123, 36, 163, 195, 25, 155, 56, 40, 153, 181, 203, 251, 72, 61, 192, 244, 93, 182, 185, 234, 90, 83, 31, 189, 229, 30, 63, 41, 21, 151, 158, 146, 16, 36, 109, 70, 33, 141, 25, 98, 243, 157, 150, 185, 41, 90, 134, 208, 24, 73, 243, 107, 245, 42, 62, 153, 14, 80, 231, 129, 0, 78, 167, 94, 43, 163, 166, 126, 182, 156, 11, 28, 214, 227, 166, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 4, 41, 208, 105, 24, 158, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 182, 230, 74, 142, 198, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 224, 182, 179, 167, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 224, 182, 179, 167, 100, 0, 0, 244, 93, 182, 185, 234, 90, 83, 31, 189, 229, 30, 63, 41, 21, 151, 158, 146, 16, 36, 109, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 74, 1, 234, 1, 254, 1, 254, 44, 180, 94, 219, 69, 23, 213, 148, 122, 253, 227, 190, 171, 249, 90, 88, 37, 6, 133, 139, 124, 98, 89, 22, 0, 143, 46, 244, 6, 31, 110, 3, 96, 98, 100, 85, 71, 10, 233, 110, 0, 0, 1, 254, 1, 254, 0, 30, 0, 30, 0, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 104, 95, 141, 138, 57, 170, 145, 252, 108, 0, 0, 0, 0, 0, 0, 0, 2, 121, 143, 14, 212, 80, 198, 43, 15, 192, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], status: Revert([8, 195, 121, 160, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 110, 111, 112, 58, 32, 97, 109, 111, 117, 110, 116, 32, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) }]"
    );
}

#[tokio::test]
async fn detects_submit_with_args() {
    let handler = TestHandler {
        transactions: Vec::new(),
    };

    let mut indexer = AuroraIndexer(handler);

    run_indexer(
        &mut indexer,
        NeardataProvider::mainnet(),
        IndexerOptions {
            preprocess_transactions: Some(PreprocessTransactionsSettings {
                prefetch_blocks: 0,
                postfetch_blocks: 0,
            }),
            ..IndexerOptions::default_with_range(BlockRange::Range {
                start_inclusive: 134404191,
                end_exclusive: Some(134404193),
            })
        },
    )
    .await
    .unwrap();

    assert_eq!(
        format!("{:?}", indexer.0.transactions),
        "[AuroraTransactionEvent { block_height: 134404192, block_timestamp_nanosec: 1733542027531204539, transaction_id: DGZqUSeetUf9ytVhBK3VxVTyxobvqTZouPePhkCfcY3r, receipt_id: ExeX9sGiwuZVw5ez5R3eP2t7vgbMUbXWrj7ZKSLjKzGX, chain_id: Some(1313161554), aurora_tx_hash: \"0xe43b…81bb\", from: Address(0xfc64eb6b6358b8370179f25d930b1598450bce5e), to: Some(Address(0x5650457e6d258e96e73db6523325fa6107731b63)), value: Wei(0), input: [106, 255, 231, 164, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 74, 92, 24, 30, 20, 6, 171, 177, 96, 137, 161, 153, 44, 99, 157, 217, 122, 15, 126, 215, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 237, 29, 2, 99, 217, 240, 0, 0], status: Succeed([]) }]"
    );
}
