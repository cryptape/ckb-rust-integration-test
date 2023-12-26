use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("get_header_by_number", "[block_number]"))]
fn get_header_by_number_with_block_number(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 反序列化输入参数
    let block_number = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();

    // 调用被测试的函数
    let block = ckb_client.get_header_by_number(block_number).unwrap();

    // 反序列化和比较输出结果
    assert_eq!(block, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}

#[rstest(mock_rpc_data("get_header_by_number", "[block_number,null]"))]
#[ignore]
fn get_header_by_number_with_block_number_null(mock_rpc_data: MockRpcData) {
    let _ckb_client = mock_rpc_data.client();
    // like [block_number]
    assert!(false,"get_header_by_number/[block_number,null]")
}

#[rstest(mock_rpc_data("get_header_by_number", "[block_number,verbosity=0]"))]
fn get_header_by_number_with_block_number_verbosity_0(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 反序列化输入参数
    let block_number = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();

    // 调用被测试的函数
    let block = ckb_client.get_packed_header_by_number(block_number).unwrap();

    // 比较输出结果
    assert_eq!(block, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}

#[rstest(mock_rpc_data("get_header_by_number", "[block_number,verbosity=1]"))]
#[ignore]
fn get_header_by_number_with_block_number_verbosity_1(mock_rpc_data: MockRpcData) {
    let _ckb_client = mock_rpc_data.client();
    // like [block_number]
    assert!(false,"get_header_by_number/[block_number,verbosity=1]")

    // 反序列化输入参数
    // let block_number = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();
    // let verbosity = "0x1";

    // 调用被测试的函数
    // let block = ckb_client.get_header_by_number_with_verbosity(block_number, verbosity).unwrap();

    // 反序列化和比较输出结果
    // assert_eq!(camel_case_to_underscore(block), serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}
