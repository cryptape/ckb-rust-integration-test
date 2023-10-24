use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("truncate", "[target_tip_hash]"))]
fn truncate_with_target_tip_hash(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 在这里执行与 'truncate' 情况相关的操作
    // 请根据您的项目实际情况添加适当的操作和断言。
    let target_tip_hash = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();

    let result  = ckb_client.truncate(target_tip_hash).unwrap();
    // 示例断言：
    // assert!(some_condition);
    assert_eq!(result,());
    // 如果有任何特定的操作或断言，您需要在上述代码中添加它们。
}
