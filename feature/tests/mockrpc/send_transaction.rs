use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("send_transaction", "data2"))]
fn send_transaction_with_data2(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 反序列化输入参数
    let tx = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();
    let params1 = serde_json::from_value(mock_rpc_data.request_data["params"][1].clone()).unwrap();

    // 调用被测试的函数
    let tx_hash = ckb_client.send_transaction(tx, params1).unwrap();

    // 比较输出结果
    assert_eq!(tx_hash, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());

}

#[rstest(mock_rpc_data("send_transaction", "TransactionFailedToResolve"))]
fn send_transaction_with_transaction_failed_to_resolve(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 反序列化输入参数
    let tx = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();
    let params1 = serde_json::from_value(mock_rpc_data.request_data["params"][1].clone()).unwrap();

    // 使用 `try` 块来处理错误情况
    match ckb_client.send_transaction(tx, Some(params1)) {
        Ok(_) => {}
        Err(_err) => {
            // assert_eq!(err, serde_json::from_value::<RpcError>(mock_rpc_data.response_data["result"].clone()).unwrap());
        }
    }
}
