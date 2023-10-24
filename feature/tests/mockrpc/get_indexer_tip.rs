use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("get_indexer_tip", "[]"))]
fn get_indexer_tip_empty(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 调用被测试的函数
    let response = ckb_client.get_indexer_tip().unwrap();

    // 反序列化和比较输出结果
    assert_eq!(response.unwrap().block_hash, serde_json::from_value(mock_rpc_data.response_data["result"]["block_hash"].clone()).unwrap());
}
