use rstest::rstest;
use crate::mock_light_rpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("get_cells_capacity", "[search_key:rust]"))]
fn get_cells_capacity_search_key(mock_rpc_data: MockRpcData) {
    println!("method:{}",mock_rpc_data.method);
    println!("params:{}",mock_rpc_data.params);
    let ckb_light_client = mock_rpc_data.client();

    let ret= ckb_light_client.get_cells_capacity(
        serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap()
    ).unwrap();

    // assert_eq!(ret, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}