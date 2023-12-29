use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("remove_transaction", "[tx_hash]"))]
fn remove_transaction_with_tx_hash(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 反序列化输入参数
    let tx_hash = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();

    let response = ckb_client.remove_transaction(tx_hash).unwrap();

    assert_eq!(response, serde_json::from_value::<bool>(mock_rpc_data.response_data["result"].clone()).unwrap());
}
