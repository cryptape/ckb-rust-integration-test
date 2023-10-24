use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("get_tip_block_number", "[]"))]
fn get_tip_block_number_empty(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 调用被测试的函数
    let number = ckb_client.get_tip_block_number().unwrap();

    // 比较输出结果
    assert_eq!(number, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}
