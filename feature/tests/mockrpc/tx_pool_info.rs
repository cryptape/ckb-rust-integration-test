use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("tx_pool_info", "[]"))]
fn tx_pool_info(mock_rpc_data: MockRpcData) {
    let  ckb_client = mock_rpc_data.client();

    // 调用被测试的函数
    let info = ckb_client.tx_pool_info().unwrap();

    // 比较输出结果
    assert_eq!(info, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}
