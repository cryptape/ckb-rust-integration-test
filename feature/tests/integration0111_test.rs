
mod mockrpc;
mod common;

use ckb_jsonrpc_types::{Cycle, DeploymentPos, Transaction};
use ckb_types::H256;
use crate::common::remove_quotes;
use crate::mockrpc::mock_rpc_data;
use crate::mockrpc::MockRpcData;
use rstest::*;

#[rstest(mock_rpc_data("estimate_cycles", "[tx]"))]
fn estimate_cycles_tx(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();
    let json_str = serde_json::to_string(&mock_rpc_data.request_data["params"][0]).unwrap();
    let transaction: Transaction = serde_json::from_str(&*json_str).unwrap();
    let cycle = ckb_client.estimate_cycles(transaction).unwrap();
    let request_cycle: &Cycle = &serde_json::from_value(mock_rpc_data.response_data["result"]["cycles"].clone()).unwrap();
    assert_eq!(request_cycle.value(), cycle.cycles.value());
}

#[rstest(mock_rpc_data("get_block", "[block_hash]"))]
fn get_block_block_hash(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();
    let code_hash: H256 = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();
    let block = ckb_client.get_block(code_hash).unwrap().unwrap();
    assert_eq!("0x".to_owned() + &block.header.hash.to_string(), remove_quotes(&serde_json::to_string(
        &mock_rpc_data.response_data["result"]["header"]["hash"]).unwrap()));
}

#[rstest(mock_rpc_data("get_consensus", "[]"))]
fn get_consensus(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();
    let consensus = ckb_client.get_consensus().unwrap();
    assert_eq!(
        serde_json::to_string(&consensus.block_version).unwrap(),
        serde_json::to_string(&mock_rpc_data.response_data["result"]["block_version"]).unwrap()
    )
}

#[rstest(mock_rpc_data("get_deployments_info", "[]"))]
fn get_deployments_info(mock_rpc_data: MockRpcData) {
    let deployments_info = mock_rpc_data.client().get_deployments_info().unwrap();
    assert_eq!(
        &serde_json::to_string(&mock_rpc_data.response_data["result"]["deployments"]["testdummy"]["state"]).unwrap(),
        &serde_json::to_string(&deployments_info.deployments.get(&DeploymentPos::Testdummy).unwrap().state).unwrap()
    )
}
