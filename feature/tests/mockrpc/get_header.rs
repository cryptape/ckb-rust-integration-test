use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("get_header", "[block_hash,verbosity=0]"))]
fn get_header_with_block_hash_verbosity_0(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 反序列化输入参数
    let block_hash = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();

    // 调用被测试的函数
    let block = ckb_client.get_packed_header(block_hash).unwrap();

    // 比较输出结果
    assert_eq!(block, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}

#[rstest(mock_rpc_data("get_header", "[block_hash,verbosity=1]"))]
#[ignore]
fn get_header_with_block_hash_verbosity_1(mock_rpc_data: MockRpcData) {
    let _ckb_client = mock_rpc_data.client();
    // like [block_hash]

    // assert!(false,"get_header/[block_hash,verbosity=1]")
    // 反序列化输入参数
    // let block_hash = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();
    // let verbosity = "0x1";
    //
    // // 调用被测试的函数
    // let block = ckb_client.get_header(block_hash, verbosity).unwrap();
    //
    // // 反序列化和比较输出结果
    // assert_eq!(camel_case_to_underscore(block), serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}

#[rstest(mock_rpc_data("get_header", "[block_hash]"))]
fn get_header_with_block_hash(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 反序列化输入参数
    let block_hash = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();

    // 调用被测试的函数
    let block = ckb_client.get_header(block_hash).unwrap();

    // 比较输出结果
    assert_eq!(block.unwrap().inner.extra_hash, serde_json::from_value(mock_rpc_data.response_data["result"]["extra_hash"].clone()).unwrap());
}
