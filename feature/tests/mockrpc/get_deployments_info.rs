use ckb_jsonrpc_types::DeploymentPos;
use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("get_deployments_info", "[]"))]
fn get_deployments_info(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 调用被测试的函数
    let deployments_info = ckb_client.get_deployments_info().unwrap();
    assert_eq!(
        deployments_info.deployments.get(&DeploymentPos::Testdummy).unwrap().threshold.numer,
        serde_json::from_value(mock_rpc_data.response_data["result"]["deployments"]["testdummy"]["threshold"]["numer"].clone()).unwrap()
    );
}
