use ckb_types::H256;
use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

// TODO support  get_packed_block
#[rstest(mock_rpc_data("get_block", "[block_hash]"))]
fn get_block_with_block_hash(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    let block_hash: H256 = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();

    let block = ckb_client.get_block(block_hash).unwrap();

    assert_eq!(
        block.clone().unwrap().header.hash.to_string(),
        mock_rpc_data.response_data["result"]["header"]["hash"].as_str().unwrap()[2..]
    );
    assert_eq!(block.unwrap().proposals.len(), mock_rpc_data.response_data["result"]["proposals"].as_array().unwrap().len());
}

#[rstest(mock_rpc_data("get_block", "[block_hash,verbosiby=2,with_cycles=True]"))]
fn get_block_with_block_hash_verbosiby_2_with_cycles_true(mock_rpc_data: MockRpcData) {
    let _ckb_client = mock_rpc_data.client();
    assert!(false,"not support: get_block/[block_hash,verbosiby=2,with_cycles=True]")

}


#[rstest(mock_rpc_data("get_block", "[block_hash,null,with_cycles=True]"))]
fn get_block_with_block_hash_null_with_cycles_true(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();
    let block_hash: H256 = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();
    let (block,cycle) = ckb_client.get_block_with_cycles(block_hash).unwrap().unwrap();
    assert_eq!(block, serde_json::from_value(mock_rpc_data.response_data["result"]["block"].clone()).unwrap());
    assert_eq!(cycle.len(), mock_rpc_data.response_data["result"]["cycle"].as_array().unwrap().len());
}


#[rstest(mock_rpc_data("get_block", "[block_hash,verbosity=0]"))]
fn get_block_with_block_hash_verbosity_0(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();
    let block_hash: H256 = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();
    let block = ckb_client.get_packed_block(block_hash).unwrap();
    assert_eq!(block.unwrap(), serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}

#[rstest(mock_rpc_data("get_block", "[block_hash,verbosity=2]"))]
fn get_block_with_block_hash_verbosity_2(mock_rpc_data: MockRpcData) {
    let _ckb_client = mock_rpc_data.client();
    assert!(false,"not support: get_block/[block_hash,verbosiby=2]")
}

#[rstest(mock_rpc_data("get_block", "data2"))]
fn get_block_with_data2(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    let block_hash: H256 = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();
    let block = ckb_client.get_block(block_hash).unwrap();

    assert_eq!(
        block.unwrap().
            transactions.get(1).unwrap().inner
            .outputs[0].lock.hash_type.to_string(),
        "data2",
    );
}

#[rstest(mock_rpc_data("get_block", "extension2"))]
fn get_block_with_extension2(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();
    let block_hash: H256 = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();
    let block = ckb_client.get_block(block_hash).unwrap();
    assert_eq!(block.unwrap().extension.unwrap(), serde_json::from_value(mock_rpc_data.response_data["result"]["extension"].clone()).unwrap());
}
