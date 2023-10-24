use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("dry_run_transaction", "[tx]"))]
fn dry_run_transaction_with_tx(mock_rpc_data: MockRpcData) {
    let _ckb_client = mock_rpc_data.client();
    let _json_str = serde_json::to_string(&mock_rpc_data.request_data["params"][0]).unwrap();
    assert!(false,"not support: dry_run_transaction/[tx]")
}