use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("clear_tx_pool", "[]"))]
fn clear_tx_pool(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();
    ckb_client.clear_tx_pool().unwrap();
}