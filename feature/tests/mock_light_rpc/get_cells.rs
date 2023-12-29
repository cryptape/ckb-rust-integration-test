use rstest::rstest;
use crate::mock_light_rpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("get_cells", "[search_key,order,limit,after_cursor]:rust"))]
fn get_cells_search_key_order_limit_after_cursor(mock_rpc_data: MockRpcData) {
    println!("method:{}",mock_rpc_data.method);
    println!("params:{}",mock_rpc_data.params);
    let ckb_light_client = mock_rpc_data.client();

    let ret= ckb_light_client.get_cells(
        serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap(),
        serde_json::from_value(mock_rpc_data.request_data["params"][1].clone()).unwrap(),
        serde_json::from_value(mock_rpc_data.request_data["params"][2].clone()).unwrap(),
        None

    ).unwrap();

    // assert_eq!(ret, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}