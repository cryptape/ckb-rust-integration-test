use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("get_raw_tx_pool", "[]"))]
fn get_raw_tx_pool_empty(mock_rpc_data: MockRpcData) {
    let _ckb_client = mock_rpc_data.client();
    assert!(false,"get_raw_tx_pool/[]")
    // 调用被测试的函数
    // let response = ckb_client.get_raw_tx_pool().unwrap();

    // 反序列化和比较输出结果
    // assert_eq!(response, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}

#[rstest(mock_rpc_data("get_raw_tx_pool", "[null]"))]
fn get_raw_tx_pool_with_null(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 调用被测试的函数
    let response = ckb_client.get_raw_tx_pool(None).unwrap();

    // 反序列化和比较输出结果
    assert_eq!(response, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}

#[rstest(mock_rpc_data("get_raw_tx_pool", "[verbose=false]"))]
fn get_raw_tx_pool_verbose_false(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 调用被测试的函数
    let response = ckb_client.get_raw_tx_pool(Some(false)).unwrap();

    // 反序列化和比较输出结果
    assert_eq!(response, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}

#[rstest(mock_rpc_data("get_raw_tx_pool", "[verbose=true]"))]
fn get_raw_tx_pool_verbose_true(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 调用被测试的函数
    let response = ckb_client.get_raw_tx_pool(Some(true)).unwrap();

    
    // 反序列化和比较输出结果
    assert_eq!(response, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}
