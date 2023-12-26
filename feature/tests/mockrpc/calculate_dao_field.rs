use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("calculate_dao_field", "[block_template]"))]
fn calculate_dao_field_with_block_template(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();
    let ret = ckb_client.calculate_dao_field(
        serde_json::from_value(
            mock_rpc_data.request_data["params"][0].clone()
        ).unwrap()).unwrap();

    assert_eq!(ret, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());

}