use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("get_block_median_time", "[block_hash]"))]
fn get_block_median_time_block_hash(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    let block_hash = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();
    let response = ckb_client.get_block_median_time(block_hash).unwrap();

    assert_eq!(response, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}
