use ckb_types::h256;
use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("sync_state", "[]"))]
fn sync_state(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 调用被测试的函数
    let state = ckb_client.sync_state().unwrap();

    // 比较输出结果
    assert_eq!(state, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
    assert!(state.assume_valid_target_reached);
    assert_eq!(serde_json::to_string(&state.assume_valid_target).unwrap(), mock_rpc_data.response_data["result"]["assume_valid_target"].to_string());
    assert_eq!(serde_json::to_string(&state.unverified_tip_hash).unwrap(), mock_rpc_data.response_data["result"]["unverified_tip_hash"].to_string());
    assert_eq!(serde_json::to_string(&state.unverified_tip_number).unwrap(), mock_rpc_data.response_data["result"]["unverified_tip_number"].to_string());
}
