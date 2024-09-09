use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("send_test_transaction", "[tx,outputs_validator]"))]
#[ignore]
fn send_transaction_with_data2(mock_rpc_data: MockRpcData) {
    //
    // let ckb_client = mock_rpc_data.client();
    //
    // // 反序列化输入参数
    // let tx = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();
    // let params1 = serde_json::from_value(mock_rpc_data.request_data["params"][1].clone()).unwrap();

    // 调用被测试的函数
    // let tx_hash = ckb_client.send_test_(tx, params1).unwrap();

    // 比较输出结果
    // assert_eq!(tx_hash, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());

}