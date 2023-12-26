use rstest::rstest;
use crate::mock_light_rpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("get_scripts", "[]"))]
fn get_scripts(mock_rpc_data: MockRpcData) {
    println!("method:{}",mock_rpc_data.method);
    println!("params:{}",mock_rpc_data.params);
    let ckb_light_client = mock_rpc_data.client();

    let ret= ckb_light_client.get_scripts(
    ).unwrap();
    // assert_eq!(ret.unwrap(), serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}