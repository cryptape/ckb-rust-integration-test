use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("set_network_active", "[state]"))]
fn set_network_active_with_state(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 反序列化输入参数
    let state = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();

    // 调用被测试的函数
    let result = ckb_client.set_network_active(state).unwrap();

    // 比较输出结果
    assert_eq!(result, ());
}
