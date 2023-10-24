use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("clear_banned_addresses", "[]"))]
fn clear_banned_addresses(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();
    ckb_client.clear_banned_addresses().unwrap();
}