use ckb_sdk::rpc::ckb_indexer::{Cell, Pagination};
use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("get_cells", "[search_key,order,limit]"))]
#[ignore]
fn get_cells_with_search_key_order_limit(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 反序列化输入参数
    let search_key = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();
    let order = serde_json::from_value(mock_rpc_data.request_data["params"][1].clone()).unwrap();
    let limit = serde_json::from_value(mock_rpc_data.request_data["params"][2].clone()).unwrap();

    // 调用被测试的函数
    let response = ckb_client.get_cells(search_key, order, limit, None).unwrap();
    let p_cells: Pagination<Cell> = serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap();
    // 反序列化和比较输出结果
    assert_eq!(
        response.objects.len(),
        p_cells.objects.len()
    );
}

// ERROR in index: Request data does not match with the data:
//  'expected request':{'id': 42, 'jsonrpc': '2.0', 'method': 'get_cells', 'params': [{'script': {'args': '0x8883a512ee2383c01574a328f60eeccbb4d78240', 'code_hash': '0x9bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce8', 'hash_type': 'type'}, 'script_type': 'lock', 'script_search_mode': 'prefix'}, 'asc', '0x2', '0x409bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce8018883a512ee2383c01574a328f60eeccbb4d78240000000000000000d0000000000000000']}
//  sdk post':{'id': 0, 'jsonrpc': '2.0', 'method': 'get_cells', 'params': [{'filter': None, 'group_by_transaction': None, 'script': {'args': '0x8883a512ee2383c01574a328f60eeccbb4d78240', 'code_hash': '0x9bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce8', 'hash_type': 'type'}, 'script_search_mode': 'prefix', 'script_type': 'lock', 'with_data': None}, 'asc', '0x2', '0x409bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce8018883a512ee2383c01574a328f60eeccbb4d78240000000000000000d0000000000000000']}
#[rstest(mock_rpc_data("get_cells", "[search_key,order,limit,after]"))]
#[ignore]
fn get_cells_with_search_key_order_limit_after(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 反序列化输入参数
    let search_key = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();
    let order = serde_json::from_value(mock_rpc_data.request_data["params"][1].clone()).unwrap();
    let limit = serde_json::from_value(mock_rpc_data.request_data["params"][2].clone()).unwrap();
    let after = serde_json::from_value(mock_rpc_data.request_data["params"][3].clone()).unwrap();

    // 调用被测试的函数
    let response = ckb_client.get_cells(search_key, order, limit, after).unwrap();
    let p_cells: Pagination<Cell> = serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap();

    // 反序列化和比较输出结果
    assert_eq!(
        response.objects.len(),
        p_cells.objects.len()
    );
}
