use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("calculate_dao_field", "[block_template]"))]
fn add_node_peer_id_address(mock_rpc_data: MockRpcData) {
    let _ckb_client = mock_rpc_data.client();
    assert!(false,"not support:calculate_dao_field[block_template]");
}