use ckb_jsonrpc_types::Transaction;
use ckb_sdk::CkbRpcClient;
use ckb_types::{H256, h256};
use crate::common::remove_quotes;

mod mockrpc;
mod common;


#[test]
#[ignore]
fn test_estimate_cycles() {
    // 获取方法名和参数
    let method = "estimate_cycles";
    let params = "[tx]";

    // 调用 mock_rpc 函数获取结果
    let result = mockrpc::get_mock_test_data(method, params).unwrap();
    let mut ckb_client = CkbRpcClient::new(&*result.url);
    let json_str = serde_json::to_string(&result.request_data["params"][0]).unwrap();
    let transaction: Transaction = serde_json::from_str(&*json_str).unwrap();
    // Now you can use the `transaction` variable as needed
    println!("{:?}", transaction);
    // Call the synchronous method within an asynchronous context
    let cycles = ckb_client.estimate_cycles(transaction).unwrap().cycles.value();
    println!("{:#?}", cycles);

    match method {
        "estimate_cycles" => {
            println!(
                "Formatted Request Data:\n{}, Formatted Response Data:\n{}",
                serde_json::to_string_pretty(&result.request_data).unwrap(),
                serde_json::to_string_pretty(&result.response_data).unwrap()

            );
        }
        _ => {
            assert_eq!(format!("0x{:x}", cycles), result.response_data["result"]["cycles"]);
            // 处理未知方法名的情况
            panic!("Unknown method: {}", method.to_string());
        }
    }
}

#[test]
#[ignore]
fn test_get_block() {
    // 获取方法名和参数
    let method = "get_block";
    let params = "[block_hash]";
    let params2 = "data2";
    // 调用 mock_rpc 函数获取结果
    let result = mockrpc::get_mock_test_data(method, params).unwrap();
    // let result2 = mockrpc::get_mock_test_data(method, params2).unwrap();
    let mut ckb_client = CkbRpcClient::new(&*result.url);
    // let mut ckb_client2 = CkbRpcClient::new(&*result2.url);
    const CODE_HASH: H256 = h256!("0xa5f5c85987a15de25661e5a214f2c1449cd803f071acc7999820f25246471f40");
    let block = ckb_client.get_block(CODE_HASH).unwrap().unwrap();
    // let block2 = ckb_client2.get_block(CODE_HASH).unwrap().unwrap();
    println!("{:#?}", block);
    // println!("{:#?}", block2);
    assert_eq!("0x".to_owned() + &block.header.hash.to_string(), remove_quotes(&serde_json::to_string(
        &result.response_data["result"]["header"]["hash"]).unwrap()));
}

#[test]
#[ignore]
fn test_get_consensus() {
    // 获取方法名和参数
    let method = "get_consensus";
    let params = "[]";
    // 调用 mock_rpc 函数获取结果
    let result = mockrpc::get_mock_test_data(method, params).unwrap();
    let mut ckb_client = CkbRpcClient::new(&*result.url);
    let consensus = ckb_client.get_consensus();
    println!("{:#?}", consensus);
}

#[test]
// #[ignore]
fn test_get_deployments_info() {
    // 获取方法名和参数
    let method = "get_deployments_info";
    let params = "[]";
    // 调用 mock_rpc 函数获取结果
    let result = mockrpc::get_mock_test_data(method, params).unwrap();
    println!("{:#?}", result.response_data);
    let mut ckb_client = CkbRpcClient::new(&*result.url);
    let deployments_info = ckb_client.get_deployments_info();
    println!("{:#?}", deployments_info);
}