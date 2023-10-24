use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};


#[rstest(mock_rpc_data("get_live_cell", "[out_point,with_data=false]"))]
fn get_live_cell_with_out_point_with_data_false(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 反序列化输入参数
    let out_point = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();
    let with_data = serde_json::from_value(mock_rpc_data.request_data["params"][1].clone()).unwrap();

    // 调用被测试的函数
    let response = ckb_client.get_live_cell(out_point, with_data).unwrap();

    // 反序列化和比较输出结果
    assert_eq!(response.status, mock_rpc_data.response_data["result"]["status"].clone().as_str().unwrap().to_string());
}

#[rstest(mock_rpc_data("get_live_cell", "[out_point,with_data=true]"))]
fn get_live_cell_with_out_point_with_data_true(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 反序列化输入参数
    let out_point = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();
    let with_data = serde_json::from_value(mock_rpc_data.request_data["params"][1].clone()).unwrap();

    // 调用被测试的函数
    let response = ckb_client.get_live_cell(out_point, with_data).unwrap();

    // 反序列化和比较输出结果
    assert_eq!(response.status, mock_rpc_data.response_data["result"]["status"].clone().as_str().unwrap().to_string());
}
