use rstest::rstest;
use crate::mock_light_rpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("local_node_info", "[]"))]
fn local_node_info(mock_rpc_data: MockRpcData) {
    println!("method:{}",mock_rpc_data.method);
    println!("params:{}",mock_rpc_data.params);
    let ckb_light_client = mock_rpc_data.client();

    let ret= ckb_light_client.local_node_info().unwrap();
    // println!("node_id:{}",ret.node_id)


    // assert_eq!(ret, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}