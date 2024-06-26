use std::fmt::{Debug, format};
use ckb_jsonrpc_types::EntryCompleted;
use ckb_sdk::RpcError;
use jsonrpc_core::futures_util::future::err;
use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};

#[rstest(mock_rpc_data("test_tx_pool_accept", "dup_cell_tx"))]
fn test_tx_pool_accept_dup_cell_tx(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 反序列化输入参数
    let tx = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();
    let params1 = serde_json::from_value(mock_rpc_data.request_data["params"][1].clone()).unwrap();

    // 调用被测试的函数
    match ckb_client.test_tx_pool_accept(tx, params1) {
        Ok(ok) => {
            assert!(false);
        }
        Err(err) => {
            // assert err contains PoolRejectedDuplicatedTransaction
            assert_eq!(format!("{err}").contains("PoolRejectedDuplicatedTransaction"), true);
        }
    };
}


#[rstest(mock_rpc_data("test_tx_pool_accept", "err_outputs_validator"))]
#[ignore]
fn test_tx_pool_accept_err_outputs_validator(mock_rpc_data: MockRpcData) {}


#[rstest(mock_rpc_data("test_tx_pool_accept", "min_fee_rejected"))]
fn test_tx_pool_accept_min_fee_rejected(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 反序列化输入参数
    let tx = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();
    let params1 = serde_json::from_value(mock_rpc_data.request_data["params"][1].clone()).unwrap();

    // 调用被测试的函数
    match ckb_client.test_tx_pool_accept(tx, params1) {
        Ok(ok) => {
            assert!(false);
        }
        Err(err) => {
            // assert err contains PoolRejectedDuplicatedTransaction
            assert_eq!(format!("{err}").contains("PoolRejectedTransactionByMinFeeRate"), true);
        }
    };
}


#[rstest(mock_rpc_data("test_tx_pool_accept", "normal_tx"))]
fn test_tx_pool_accept_normal_tx(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 反序列化输入参数
    let tx = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();
    let params1 = serde_json::from_value(mock_rpc_data.request_data["params"][1].clone()).unwrap();

    // 调用被测试的函数
    let response = ckb_client.test_tx_pool_accept(tx, params1).unwrap();
    assert_eq!(response, serde_json::from_value(mock_rpc_data.response_data["result"].clone()).unwrap());
}


#[rstest(mock_rpc_data("test_tx_pool_accept", "TransactionFailedToResolve"))]
fn test_tx_pool_accept_TransactionFailedToResolve(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 反序列化输入参数
    let tx = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();
    let params1 = serde_json::from_value(mock_rpc_data.request_data["params"][1].clone()).unwrap();

    // 调用被测试的函数
    match ckb_client.test_tx_pool_accept(tx, params1) {
        Ok(ok) => {
            assert!(false);
        }
        Err(err) => {
            // assert err contains PoolRejectedDuplicatedTransaction
            assert_eq!(format!("{err}").contains("TransactionFailedToResolve"), true);
        }
    };
}
// TransactionFailedToVerify

#[rstest(mock_rpc_data("test_tx_pool_accept", "TransactionFailedToVerify"))]
fn test_tx_pool_accept_TransactionFailedToVerify(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    // 反序列化输入参数
    let tx = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();
    let params1 = serde_json::from_value(mock_rpc_data.request_data["params"][1].clone()).unwrap();

    // 调用被测试的函数
    match ckb_client.test_tx_pool_accept(tx, params1) {
        Ok(ok) => {
            assert!(false);
        }
        Err(err) => {
            // assert err contains PoolRejectedDuplicatedTransaction
            assert_eq!(format!("{err}").contains("TransactionFailedToVerify"), true);
        }
    };
}