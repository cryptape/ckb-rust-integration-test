use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("get_block_template", "[]"))]
fn get_block_template(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();
    assert!(false,"get_block_template/[]")
    // let response = ckb_client.get_block_template(None, None, None).unwrap();

    // assert_eq!(response, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}


#[rstest(mock_rpc_data("get_block_template", "[null,null,null]"))]
fn get_block_template_with_null_null_null(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    let response = ckb_client.get_block_template(None, None, None).unwrap();

    assert_eq!(response, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}