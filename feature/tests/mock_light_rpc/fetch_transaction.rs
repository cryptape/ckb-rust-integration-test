use rstest::rstest;
use crate::mock_light_rpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("fetch_transaction", "fetched"))]
fn fetch_transaction_fetched(mock_rpc_data: MockRpcData) {
    println!("method:{}",mock_rpc_data.method);
    println!("params:{}",mock_rpc_data.params);
    let ckb_light_client = mock_rpc_data.client();

    let ret= ckb_light_client.fetch_transaction(
        serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap()
    ).unwrap();
    assert_eq!(ret, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}