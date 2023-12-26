use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("get_tip_header", "[]"))]
fn get_tip_header(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 调用被测试的函数
    let header = ckb_client.get_tip_header().unwrap();

    // 反序列化和比较输出结果
    assert_eq!((header), serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}

#[rstest(mock_rpc_data("get_tip_header", "[null]"))]
#[ignore]
fn get_tip_header_with_null(mock_rpc_data: MockRpcData) {
    let _client = mock_rpc_data.client();
    // like []
    assert!(false,"not support get_tip_header/[null]")
}

#[rstest(mock_rpc_data("get_tip_header", "[verbosity=0]"))]
fn get_tip_header_with_verbosity_0(mock_rpc_data: MockRpcData) {

    let ckb_client = mock_rpc_data.client();

    // 调用被测试的函数
    let header = ckb_client.get_packed_tip_header().unwrap();
    assert_eq!((header), serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());

}

#[rstest(mock_rpc_data("get_tip_header", "[verbosity=1]"))]
#[ignore]
fn get_tip_header_with_verbosity_1(mock_rpc_data: MockRpcData) {
    // 跳过测试用例
    let _client = mock_rpc_data.client();
    // like []
}
