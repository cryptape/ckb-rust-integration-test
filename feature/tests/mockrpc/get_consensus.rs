use ckb_jsonrpc_types::Consensus;
use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("get_consensus", "[]"))]
fn get_consensus_empty(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 调用被测试的函数
    let consensus = ckb_client.get_consensus().unwrap();
    let consensus_response:Consensus =  serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap();

    assert_eq!(
        consensus.block_version,
        consensus_response.block_version
    );

}
