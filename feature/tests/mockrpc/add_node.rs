use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("add_node", "[peer_id,address]"))]
fn add_node_peer_id_address(mock_rpc_data: MockRpcData) {
    println!("method:{}",mock_rpc_data.method);
    println!("params:{}",mock_rpc_data.params);
    let ckb_client = mock_rpc_data.client();
    let ret= ckb_client.add_node(
        mock_rpc_data.request_data["params"][0].as_str().unwrap().to_string(),
        mock_rpc_data.request_data["params"][1].as_str().unwrap().to_string(),
    ).unwrap();
    assert_eq!(ret,());
}