use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("get_peers", "[]"))]
fn get_peers_empty(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 调用被测试的函数
    let response = ckb_client.get_peers().unwrap();

    // 反序列化和比较输出结果
    assert_eq!(response.len(), mock_rpc_data.response_data["result"].as_array().unwrap().len());
}
