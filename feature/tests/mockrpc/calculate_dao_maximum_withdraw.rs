use ckb_jsonrpc_types::Capacity;
use futures::future::err;
use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("calculate_dao_maximum_withdraw", "[out_point,kind]"))]
fn calculate_dao_maximum_withdraw_with_out_point_kind(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();
    let ret = ckb_client.calculate_dao_maximum_withdraw(
        serde_json::from_value(
            mock_rpc_data.request_data["params"][0].clone()
        ).unwrap(),
        serde_json::from_value(
            mock_rpc_data.request_data["params"][1].clone()
        ).unwrap(),
    ).unwrap();
    assert_eq!(ret, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}

#[rstest(mock_rpc_data("calculate_dao_maximum_withdraw", "DaoError"))]
fn calculate_dao_maximum_withdraw_with_dao_error(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();
    match ckb_client.calculate_dao_maximum_withdraw(
        serde_json::from_value(
            mock_rpc_data.request_data["params"][0].clone()
        ).unwrap(),
        serde_json::from_value(
            mock_rpc_data.request_data["params"][1].clone()
        ).unwrap(),
    ) {
        Ok(_) => {
            assert!(false)
        }
        Err(err) => {
            //TODO add assert
            println!("err: {}", err)
        }
    };
}