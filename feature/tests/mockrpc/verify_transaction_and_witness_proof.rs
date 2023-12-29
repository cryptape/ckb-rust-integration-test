use ckb_types::H256;
use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("verify_transaction_and_witness_proof", "[tx_proof]"))]
fn verify_transaction_and_witness_proof_with_tx_proof(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 反序列化输入参数
    let tx_proof = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();

    // 调用被测试的函数
    let strs = ckb_client.verify_transaction_and_witness_proof(tx_proof).unwrap();

    // 比较输出结果
    assert_eq!(strs, serde_json::from_value::<Vec<H256>>(mock_rpc_data.response_data["result"].clone()).unwrap());
}
