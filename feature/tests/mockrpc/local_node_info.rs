use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("local_node_info", "[]"))]
fn local_node_info(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 调用被测试的函数
    let header = ckb_client.local_node_info().unwrap();

    // 反序列化和比较输出结果
    assert_eq!(header, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}
