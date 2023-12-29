use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("get_epoch_by_number", "[epoch_number]"))]
fn get_epoch_by_number_with_epoch_number(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 反序列化输入参数
    let epoch_number = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();

    // 调用被测试的函数
    let response = ckb_client.get_epoch_by_number(epoch_number).unwrap();

    // 比较输出结果
    assert_eq!(response, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}
