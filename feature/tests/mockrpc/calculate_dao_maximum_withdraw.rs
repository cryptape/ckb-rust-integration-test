use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("calculate_dao_maximum_withdraw", "[out_point,kind]"))]
fn calculate_dao_maximum_withdraw_with_out_point_kind(mock_rpc_data: MockRpcData) {
    let _ckb_client = mock_rpc_data.client();
    assert!(false,"not support: calculate_dao_maximum_withdraw/[out_point,kind]")
}

#[rstest(mock_rpc_data("calculate_dao_maximum_withdraw", "DaoError"))]
fn calculate_dao_maximum_withdraw_with_dao_error(mock_rpc_data: MockRpcData) {
    let _ckb_client = mock_rpc_data.client();
    assert!(false,"not support: calculate_dao_maximum_withdraw/DaoError")
}