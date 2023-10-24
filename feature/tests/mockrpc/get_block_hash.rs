use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("get_block_hash", "[block_number]"))]
fn get_block_hash_block_number(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    let block_number = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();
    let response = ckb_client.get_block_hash(block_number).unwrap();

    assert_eq!(
        response.unwrap(),
        serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap()
    );
}

#[rstest(mock_rpc_data("get_block_hash", "null"))]
fn get_block_hash_null(mock_rpc_data: MockRpcData) {
    let _ckb_client = mock_rpc_data.client();
    assert!(false,"get_block_hash/null")
    // let response = ckb_client.get_block_hash(None).unwrap();

    // assert_eq!(response, mock_rpc_data.response_data["result"]);
}
