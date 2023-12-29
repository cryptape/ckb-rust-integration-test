use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("ping_peers", "[]"))]
fn ping_peers(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 调用被测试的函数
    let peers = ckb_client.ping_peers().unwrap();

    // 比较输出结果
    assert_eq!(peers, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}
