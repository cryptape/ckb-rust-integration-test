use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("remove_node", "[peer_id]"))]
fn remove_node_with_peer_id(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 反序列化输入参数
    let peer_id = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();

    // 调用被测试的函数
    let result = ckb_client.remove_node(peer_id).unwrap();

    // 比较输出结果
    assert_eq!(result, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}
