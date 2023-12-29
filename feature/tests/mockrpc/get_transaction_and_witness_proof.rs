use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("get_transaction_and_witness_proof", "[hashs,null]"))]
#[ignore]
fn get_transaction_and_witness_proof_with_hashs(mock_rpc_data: MockRpcData) {
    // support ,but ckb-mock-data not support
    let ckb_client = mock_rpc_data.client();
    let tx_hashes = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();
    let result = ckb_client.get_transaction_and_witness_proof(tx_hashes,None).unwrap();
    assert_eq!(result.block_hash, serde_json::from_value(mock_rpc_data.response_data["result"]["block_hash"].clone()).unwrap());
    assert_eq!(result.transactions_proof, serde_json::from_value(mock_rpc_data.response_data["result"]["transactions_proof"].clone()).unwrap());
    assert_eq!(result.witnesses_proof, serde_json::from_value(mock_rpc_data.response_data["result"]["witnesses_proof"].clone()).unwrap());
}

#[rstest(mock_rpc_data("get_transaction_and_witness_proof", "[tx_hashs,block_hash]"))]
fn get_transaction_and_witness_proof_with_tx_hashs_block_hash(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 反序列化输入参数
    let tx_hashes = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();
    let block_hash = serde_json::from_value(mock_rpc_data.request_data["params"][1].clone()).unwrap();

    // 调用被测试的函数
    let result = ckb_client.get_transaction_and_witness_proof(tx_hashes, block_hash).unwrap();

    // 比较输出结果
    assert_eq!(result.block_hash, serde_json::from_value(mock_rpc_data.response_data["result"]["block_hash"].clone()).unwrap());
    assert_eq!(result.transactions_proof, serde_json::from_value(mock_rpc_data.response_data["result"]["transactions_proof"].clone()).unwrap());
    assert_eq!(result.witnesses_proof, serde_json::from_value(mock_rpc_data.response_data["result"]["witnesses_proof"].clone()).unwrap());
}
