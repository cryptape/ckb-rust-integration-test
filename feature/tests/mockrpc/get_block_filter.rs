use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("get_block_filter", "[block_hash]"))]
fn get_block_filter_with_block_hash(mock_rpc_data: MockRpcData) {
    let _ckb_client = mock_rpc_data.client();
    assert!(false,"not support: get_block_filter/[block_hash]")

    // let block_hash = &mock_rpc_data.request_data["params"][0];
    // let response = ckb_client.get_block_filter(block_hash).unwrap();
    //
    // assert_eq!(
    //     response,
    //     mock_rpc_data.response_data["result"]
    // );
}

#[rstest(mock_rpc_data("get_block_filter", "null"))]
#[ignore]
fn get_block_filter_with_null(mock_rpc_data: MockRpcData) {
    let _ckb_client = mock_rpc_data.client();
    assert!(false,"get_block_filter/null")
    // let response = ckb_client.get_block_filter(null).unwrap();

    // assert_eq!(response, mock_rpc_data.response_data["result"]);
}
