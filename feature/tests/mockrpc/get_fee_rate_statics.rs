use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("get_fee_rate_statics", "[]"))]
#[ignore]
fn get_fee_rate_statics(mock_rpc_data: MockRpcData) {
    let _ckb_client = mock_rpc_data.client();
    // like [null]
    // assert!(false,"not support get_fee_rate_statics/[]")
    // // 调用被测试的函数
    // let statistics = ckb_client.get_fee_rate_statics(None).unwrap();
    //
    // assert_eq!(statistics, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}

#[rstest(mock_rpc_data("get_fee_rate_statics", "[target]"))]
fn get_fee_rate_statics_with_target(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 反序列化输入参数
    let target = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();

    // 调用被测试的函数
    let statistics = ckb_client.get_fee_rate_statics(target).unwrap();

    // 比较输出结果
    assert_eq!(statistics, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}

#[rstest(mock_rpc_data("get_fee_rate_statics", "[null]"))]
fn get_fee_rate_statics_with_null(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 调用被测试的函数
    let statistics = ckb_client.get_fee_rate_statics(None).unwrap();

    // 比较输出结果
    assert_eq!(statistics, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}
