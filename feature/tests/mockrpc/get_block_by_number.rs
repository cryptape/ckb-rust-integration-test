use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};


#[rstest(mock_rpc_data("get_block_by_number", "[block_number,verbosity=0,with_cycles]"))]
fn get_block_by_number_verbosity_0_with_cycles(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    let block_number = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();

    let (response_byte, cycles) = ckb_client.get_packed_block_by_number_with_cycles(block_number).unwrap().unwrap();
    assert_eq!(response_byte,
               serde_json::from_value(mock_rpc_data.response_data["result"]["block"].clone()).unwrap()
    );
    let response_cycles = mock_rpc_data.response_data["result"]["cycles"].as_array().unwrap();
    assert_eq!(cycles.len(), response_cycles.len());
}

//
#[rstest(mock_rpc_data("get_block_by_number", "[block_number]"))]
fn get_block_by_number_block_number(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    let block_number = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();
    let response = ckb_client.get_block_by_number(block_number).unwrap();
    assert_eq!(
        response.unwrap().header.hash.to_string(),
        mock_rpc_data.response_data["result"]["header"]["hash"].as_str().unwrap()[2..]
    );
}

#[rstest(mock_rpc_data("get_block_by_number", "[block_number,verbosity=0]"))]
fn get_block_by_number_verbosity_0(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();
    let block_number = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();
    let response = ckb_client.get_packed_block_by_number(block_number).unwrap();
    assert_eq!(
        response.unwrap(),
        serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap()
    )
}

#[rstest(mock_rpc_data("get_block_by_number", "[block_number,null,with_cycles]"))]
fn get_block_by_number_block_number_null_with_cycles(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();
    let block_number = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();
    let (block, cycle) = ckb_client.get_block_by_number_with_cycles(block_number).unwrap().unwrap();
    assert_eq!(
        block,
        serde_json::from_value(mock_rpc_data.response_data["result"]["block"].clone()).unwrap()
    )


    // let block_number = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();
    //
    // let response = ckb_client.get_packed_block_by_number_with_cycles(block_number).unwrap();
    //
    // assert_eq!(
    //     response.unwrap(),
    //     serde_json::from_value( mock_rpc_data.response_data["result"].clone()).unwrap()
    // )
}

#[rstest(mock_rpc_data("get_block_by_number", "[block_number,verbosity=2,with_cycles]"))]
#[ignore]
fn get_block_by_number_block_number_verbosity_2_with_cycles(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();
    let block_number = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();
    let result = ckb_client.get_block_by_number_with_cycles(block_number).unwrap();
    assert!(false, "not support: get_block_by_number/[block_number,verbosity=2,with_cycles]");

}

//
#[rstest(mock_rpc_data("get_block_by_number", "[block_number,verbosity=2]"))]
#[ignore]
fn get_block_by_number_verbosity_2(mock_rpc_data: MockRpcData) {
    let _ckb_client = mock_rpc_data.client();
    assert!(false, "not support: get_block_by_number/[block_number,verbosity=2]");
}
