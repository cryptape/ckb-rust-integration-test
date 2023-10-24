use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};



#[rstest(mock_rpc_data("generate_block", "[]"))]
fn generate_block(mock_rpc_data: MockRpcData) {
    let _client = mock_rpc_data.client();
    assert!(false,"not support: generate_block/[]")
}