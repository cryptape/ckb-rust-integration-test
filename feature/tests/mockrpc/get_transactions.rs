use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("get_transactions", "[search_key,order,limit]"))]
fn get_transactions_with_search_key_order_limit(mock_rpc_data: MockRpcData) {
    let _ckb_client = mock_rpc_data.client();
    assert!(false,"get_transactions/[search_key,order,limit]")
    // 反序列化输入参数
    // let search_key = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();
    // let order = serde_json::from_value(mock_rpc_data.request_data["params"][1].clone()).unwrap();
    // let limit = serde_json::from_value(mock_rpc_data.request_data["params"][2].clone()).unwrap();
    //
    // // 调用被测试的函数
    // let result = ckb_client.get_transactions(search_key, order, limit,None).unwrap();
    //
    // // 反序列化和比较输出结果
    // assert_eq!(result.objects.len(),mock_rpc_data.response_data["result"]["objects"].as_array().unwrap().len());
}


// TODO fix : ERROR in index: Request data does not match with the data:
//  'expected request':{'id': 42, 'jsonrpc': '2.0', 'method': 'get_transactions', 'params': [{'script': {'code_hash': '0x9bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce8', 'hash_type': 'type', 'args': '0x5989ae415bb667931a99896e5fbbfad9ba53a223'}, 'script_type': 'lock', 'script_search_mode': 'prefix'}, 'asc', '0x64', None]}
//  sdk post'         :{'id': 0, 'jsonrpc': '2.0', 'method': 'get_transactions', 'params': [{'filter': None, 'group_by_transaction': None, 'script': {'args': '0x5989ae415bb667931a99896e5fbbfad9ba53a223', 'code_hash': '0x9bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce8', 'hash_type': 'type'}, 'script_search_mode': 'prefix', 'script_type': 'lock', 'with_data': None}, 'asc', '0x64', None]}
#[rstest(mock_rpc_data("get_transactions", "[search_key,order,limit,null]"))]
#[ignore]
fn get_transactions_with_search_key_order_limit_null(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 反序列化输入参数
    let search_key = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();
    let order = serde_json::from_value(mock_rpc_data.request_data["params"][1].clone()).unwrap();
    let limit = serde_json::from_value(mock_rpc_data.request_data["params"][2].clone()).unwrap();

    // 调用被测试的函数
    let result = ckb_client.get_transactions(search_key, order, limit,None).unwrap();

    // 反序列化和比较输出结果
    assert_eq!(result.objects.len(),mock_rpc_data.response_data["result"]["objects"].as_array().unwrap().len());
}


// [2023-10-20 11:15:49,620] ERROR in index: Request data does not match with the data:
//  'expected request':{'id': 42, 'jsonrpc': '2.0', 'method': 'get_transactions', 'params': [{'script': {'code_hash': '0x9bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce8', 'hash_type': 'type', 'args': '0x5989ae415bb667931a99896e5fbbfad9ba53a223'}, 'script_type': 'lock', 'script_search_mode': 'prefix'}, 'asc', '0x2', '0x809bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce8015989ae415bb667931a99896e5fbbfad9ba53a22300000000005b0671000000010000000000']}
//  sdk post':{'id': 0, 'jsonrpc': '2.0', 'method': 'get_transactions', 'params': [{'filter': None, 'group_by_transaction': None, 'script': {'args': '0x5989ae415bb667931a99896e5fbbfad9ba53a223', 'code_hash': '0x9bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce8', 'hash_type': 'type'}, 'script_search_mode': 'prefix', 'script_type': 'lock', 'with_data': None}, 'asc', '0x2', '0x809bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce8015989ae415bb667931a99896e5fbbfad9ba53a22300000000005b0671000000010000000000']}
#[rstest(mock_rpc_data("get_transactions", "[search_key,order,limit,after]"))]
#[ignore]
fn get_transactions_with_search_key_order_limit_after(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 反序列化输入参数
    let search_key = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();
    let order = serde_json::from_value(mock_rpc_data.request_data["params"][1].clone()).unwrap();
    let limit = serde_json::from_value(mock_rpc_data.request_data["params"][2].clone()).unwrap();
    let after = serde_json::from_value(mock_rpc_data.request_data["params"][3].clone()).unwrap();

    // 调用被测试的函数
    let result = ckb_client.get_transactions(search_key, order, limit, after).unwrap();

    // 反序列化和比较输出结果
    assert_eq!(result.objects.len(),mock_rpc_data.response_data["result"]["objects"].as_array().unwrap().len());
}
