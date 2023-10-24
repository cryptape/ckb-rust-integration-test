use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("get_fee_rate_statistics", "[]"))]
fn get_fee_rate_statistics(mock_rpc_data: MockRpcData) {
    let _ckb_client = mock_rpc_data.client();
    assert!(false,"not support get_fee_rate_statistics/[]")
    // 调用被测试的函数
    // let statistics = ckb_client.get_fee_rate_statistics().unwrap();

    // 比较输出结果
    // assert_eq!(statistics, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}

#[rstest(mock_rpc_data("get_fee_rate_statistics", "[target]"))]
fn get_fee_rate_statistics_with_target(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 反序列化输入参数
    let target = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();

    // 调用被测试的函数
    let statistics = ckb_client.get_fee_rate_statistics(target).unwrap();

    // 比较输出结果
    assert_eq!(statistics, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}


// TODO check result can be null  bug:  FeeRateStatistics should option(FeeRateStatistics)
#[rstest(mock_rpc_data("get_fee_rate_statistics", "null"))]
#[ignore]
fn get_fee_rate_statistics_with_null(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 调用被测试的函数
    let statistics = ckb_client.get_fee_rate_statistics(None).unwrap();

    // 比较输出结果
    assert_eq!(statistics, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}