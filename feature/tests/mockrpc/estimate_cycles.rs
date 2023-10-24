use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};
use ckb_jsonrpc_types::{Cycle, Transaction};

#[rstest(mock_rpc_data("estimate_cycles", "[tx]"))]
fn estimate_cycles_with_tx(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    let transaction: Transaction = serde_json::from_value(
        mock_rpc_data.request_data["params"][0].clone()
    ).unwrap();

    let cycle = ckb_client.estimate_cycles(transaction).unwrap();
    let request_cycle: &Cycle = &serde_json::from_value(mock_rpc_data.response_data["result"]["cycles"].clone()).unwrap();
    assert_eq!(request_cycle.value(), cycle.cycles.value());
}