mod common;

use ckb_jsonrpc_types::{Transaction, Uint64};
use ckb_sdk::CkbRpcClient;

#[test]
fn test_getTransaction() {
    let header = CkbRpcClient::get_tip_header(&mut CkbRpcClient::new(common::CKB_DEVNET)).unwrap().inner;
    let block = CkbRpcClient::get_block_by_number(&mut CkbRpcClient::new(common::CKB_DEVNET),
    header.number);
    let hash = block.unwrap().unwrap().transactions[0].hash.clone();
    println!("Hash:{:#?}", hash);
    println!("Hash:{:#?}", hash.to_string());
    let res = CkbRpcClient::get_transaction(&mut CkbRpcClient::new(common::CKB_DEVNET), hash).unwrap();
    println!("TransactionWithStatusResponse:{:#?}", res);
    assert_eq!(res.unwrap().cycles.ok_or(0), Err(0), "cycle is None");
}

#[test]
fn test_getBlock(){
    let header = CkbRpcClient::get_tip_header(&mut CkbRpcClient::new(common::CKB_DEVNET)).unwrap().inner;
    let block = CkbRpcClient::get_block_by_number(&mut CkbRpcClient::new(common::CKB_DEVNET),
                                                  header.number);
    let hash = block.unwrap().unwrap().transactions[0].hash.clone();
    println!("Hash:{:#?}", hash);
    println!("Hash:{:#?}", hash.to_string());
    let res = CkbRpcClient::get_block(&mut CkbRpcClient::new(common::CKB_DEVNET), hash).unwrap();
    println!("TransactionWithStatusResponse:{:#?}", res);
}

#[test]
fn test_getBlockByNumber(){
    let header = CkbRpcClient::get_tip_header(&mut CkbRpcClient::new(common::CKB_DEVNET)).unwrap().inner;
    let res = CkbRpcClient::get_block_by_number(&mut CkbRpcClient::new(common::CKB_DEVNET), 0.into()).unwrap();
    let block = CkbRpcClient::get_block_by_number(&mut CkbRpcClient::new(common::CKB_DEVNET),
                                                  header.number);
    let hash = block.unwrap().unwrap().transactions[0].hash.clone();
    println!("Hash:{:#?}", hash);
}

#[test]
fn test_estimateCycles(){
    let res = CkbRpcClient::estimate_cycles(&mut CkbRpcClient::new(common::CKB_DEVNET),
    Transaction{
        version: Default::default(),
        cell_deps: vec![],
        header_deps: vec![],
        inputs: vec![],
        outputs: vec![],
        outputs_data: vec![],
        witnesses: vec![]
    }).unwrap();
    println!("{:#?}", res)
}

#[test]
fn test_getFeeRateStatics(){
    let res = CkbRpcClient::get_fee_rate_statics(&mut CkbRpcClient::new(common::CKB_DEVNET), Some(Uint64::from(1))).unwrap();
    println!("{:#?}", res);
    // assert_ne!(res.mean.value(), 0);
    // assert_ne!(res.median.value(), 0);
}