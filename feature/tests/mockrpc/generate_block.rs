use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};


#[rstest(mock_rpc_data("generate_block", "[]"))]
fn generate_block(mock_rpc_data: MockRpcData) {
    let client = mock_rpc_data.client();
    let block = client.generate_block().unwrap();
    assert_eq!(block, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}