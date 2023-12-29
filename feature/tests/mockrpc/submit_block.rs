use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("submit_block", "[work_id,block]"))]
fn submit_block_with_work_id_and_block(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();


    let work_id = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();
    let block = serde_json::from_value(mock_rpc_data.request_data["params"][1].clone()).unwrap();

    let hash = ckb_client.submit_block(work_id,block).unwrap();

    assert_eq!(hash.to_string(), mock_rpc_data.response_data["result"].clone().as_str().unwrap()[2..]);
}
