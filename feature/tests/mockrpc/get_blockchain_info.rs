use ckb_jsonrpc_types::ChainInfo;
use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("get_blockchain_info", "[]"))]
fn get_blockchain_info_empty(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();
    let response = ckb_client.get_blockchain_info().unwrap();
    let info:ChainInfo = serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap();

    assert_eq!(
        response.chain,
        info.chain
    );

    assert_eq!(
        response.difficulty,
        info.difficulty
    );
}
