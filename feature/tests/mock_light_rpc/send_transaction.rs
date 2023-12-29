use rstest::rstest;
use crate::mock_light_rpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("send_transaction", "[tx]"))]
fn send_transaction(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 反序列化输入参数
    let tx = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();

    // 调用被测试的函数
    let tx_hash = ckb_client.send_transaction(tx).unwrap();

    // 比较输出结果
    assert_eq!(tx_hash, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());

}
