use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("tx_pool_ready", "[]"))]
fn tx_pool_ready_empty_params(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    let result = ckb_client.tx_pool_ready().unwrap();
    assert_eq!(result,true);
    // 在这里执行与 'tx_pool_ready' 情况相关的操作
    // 请根据您的项目实际情况添加适当的操作和断言。

    // 示例断言：
    // assert!(some_condition);

    // 如果有任何特定的操作或断言，您需要在上述代码中添加它们。
}
