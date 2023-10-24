use ckb_jsonrpc_types::Status;
use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("get_transaction", "[tx_hash,verbosity=0]"))]
fn get_transaction_with_verbosity_0(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 反序列化输入参数
    let tx_hash = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();

    // 调用被测试的函数
    let tx = ckb_client.get_packed_transaction(tx_hash).unwrap();

    // 比较输出结果
    assert_eq!(tx.transaction, serde_json::from_value(mock_rpc_data.response_data["result"]["transaction"].clone()).unwrap());
}

#[rstest(mock_rpc_data("get_transaction", "[tx_hash,verbosity,only_committed=null]"))]
fn get_transaction_with_verbosity_and_only_committed_null(mock_rpc_data: MockRpcData) {
    let _ckb_client = mock_rpc_data.client();

    assert!(false,"get_transaction/[tx_hash,verbosity,only_committed=null]")
    // 反序列化输入参数
    // let tx_hash = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();
    // let verbosity = serde_json::from_value(mock_rpc_data.request_data["params"][1].clone()).unwrap();
    // let only_committed = serde_json::from_value(mock_rpc_data.request_data["params"][2].clone()).unwrap();

    // 调用被测试的函数
    // let tx = ckb_client.get_transaction(tx_hash).unwrap();

    // 比较输出结果
    // assert_eq!(tx.tx_status.block_hash, serde_json::from_value(mock_rpc_data.response_data["result"]["tx_status"]["block_hash"].clone()).unwrap());
}

#[rstest(mock_rpc_data("get_transaction", "[tx_hash,verbosity=null,only_committed=true]"))]
fn get_transaction_with_verbosity_null_and_only_committed_true(mock_rpc_data: MockRpcData) {
    let _ckb_client = mock_rpc_data.client();
    assert!(false,"get_transaction/[tx_hash,verbosity=null,only_committed=true]")

//
//     // 反序列化输入参数
//     let tx_hash = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();
//     let only_committed = serde_json::from_value(mock_rpc_data.request_data["params"][2].clone()).unwrap();
//
//     // 调用被测试的函数
//     let tx = ckb_client.get_transaction(tx_hash, only_committed).unwrap();
//
//     // 比较输出结果
//     assert_eq!(tx.tx_status.block_hash, serde_json::from_value(mock_rpc_data.response_data["result"]["tx_status"]["block_hash"].clone()).unwrap());
}

#[rstest(mock_rpc_data("get_transaction", "time_added_to_pool"))]
fn get_transaction_with_time_added_to_pool(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 反序列化输入参数
    let tx_hash = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();

    // 调用被测试的函数
    let tx = ckb_client.get_transaction(tx_hash).unwrap();

    // 比较输出结果
    assert_eq!(tx.unwrap().time_added_to_pool.unwrap(), serde_json::from_value(mock_rpc_data.response_data["result"]["time_added_to_pool"].clone()).unwrap());

}

#[rstest(mock_rpc_data("get_transaction", "data2"))]
fn get_transaction_with_data2(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 反序列化输入参数
    let tx_hash = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();

    // 调用被测试的函数
    let _tx = ckb_client.get_transaction(tx_hash).unwrap();

    // 比较输出结果

    // assert_eq!(tx.cycles, serde_json::from_value(mock_rpc_data.response_data["result"]["cycles"].clone()).unwrap());
}

#[rstest(mock_rpc_data("get_transaction", "rejected"))]
fn get_transaction_with_rejected(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 反序列化输入参数
    let tx_hash = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();

    // 调用被测试的函数
    let tx = ckb_client.get_transaction(tx_hash).unwrap();

    // 比较输出结果
    assert_eq!(tx.unwrap().tx_status.status, Status::Rejected);
    // assert_eq!(tx.unwrap().tx_status.reason, serde_json::from_value(mock_rpc_data.response_data["result"]["tx_status"]["reason"].clone()).unwrap());
}
