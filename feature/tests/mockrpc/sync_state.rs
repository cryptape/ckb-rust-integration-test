use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("sync_state", "[]"))]
fn sync_state(mock_rpc_data: MockRpcData) {
    let  ckb_client = mock_rpc_data.client();

    // 调用被测试的函数
    let state = ckb_client.sync_state().unwrap();

    // 比较输出结果
    assert_eq!(state, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}
