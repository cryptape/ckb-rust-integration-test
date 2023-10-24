use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("set_ban", "[address,command,ban_time,absolute,reason]"))]
fn set_ban_with_params(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 反序列化输入参数
    let address = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();
    let command = serde_json::from_value(mock_rpc_data.request_data["params"][1].clone()).unwrap();
    let ban_time = serde_json::from_value(mock_rpc_data.request_data["params"][2].clone()).unwrap();
    let absolute = serde_json::from_value(mock_rpc_data.request_data["params"][3].clone()).unwrap();
    let reason = serde_json::from_value(mock_rpc_data.request_data["params"][4].clone()).unwrap();

    // 调用被测试的函数
    let ban = ckb_client.set_ban(address, command, ban_time, absolute, reason).unwrap();

    // 比较输出结果
    assert_eq!(ban, ());
}
