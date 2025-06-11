use crate::dev::config;
use ckb_sdk::rpc::LightClientRpcClient;
use ckb_sdk::CkbRpcClient;

#[test]
fn test_get_tip_block_number() {
    let tip_number =
        CkbRpcClient::get_tip_block_number(&mut CkbRpcClient::new(config::CKB_DEV_URL)).unwrap();
    assert!(
        tip_number.value() > 0,
        "Tip block number should be greater than 0"
    );
}

#[test]
fn test_light_client_get_tip_block_number() {
    let client = LightClientRpcClient::new(config::CKB_LIGHT_CLIENT_URL);
    let header = client.get_tip_header().unwrap();
    assert!(
        header.inner.number.value() > 0,
        "Tip block number should be greater than 0"
    );
}
