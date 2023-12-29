use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("get_transaction_proof", "[tx_hashes,block_hash]"))]
fn get_transaction_proof_with_tx_hashes_block_hash(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 反序列化输入参数
    let tx_hashes = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();
    let block_hash = serde_json::from_value(mock_rpc_data.request_data["params"][1].clone()).unwrap();

    // 调用被测试的函数
    let result = ckb_client.get_transaction_proof(tx_hashes, block_hash).unwrap();

    // 反序列化和比较输出结果
    assert_eq!(result, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}

#[rstest(mock_rpc_data("get_transaction_proof", "[tx_hashes,null]"))]
fn get_transaction_proof_with_tx_hashes_null(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 反序列化输入参数
    let tx_hashes = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();

    // 调用被测试的函数
    let result = ckb_client.get_transaction_proof(tx_hashes,None).unwrap();

    // 反序列化和比较输出结果
    assert_eq!(result, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}

#[rstest(mock_rpc_data("get_transaction_proof", "[tx_hashes]"))]
#[ignore]
fn get_transaction_proof_with_tx_hashes(mock_rpc_data: MockRpcData) {
    let _ckb_client = mock_rpc_data.client();
    // like [tx_hashes,null]
    assert!(false,"not support: get_transaction_proof/[tx_hashes]")
    // 反序列化输入参数
    // let tx_hashes = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();

    // 调用被测试的函数
    // let result = ckb_client.get_transaction_proof(tx_hashes).unwrap();

    // 反序列化和比较输出结果
    // assert_eq!(result, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}
