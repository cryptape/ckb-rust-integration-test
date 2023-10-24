use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};


// [2023-10-20 11:47:04,120] ERROR in index: Request data does not match with the data:
//  'expected request':{'jsonrpc': '2.0', 'method': 'send_alert', 'params': [{'id': '0x1', 'cancel': '0x0', 'priority': '0x1', 'message': 'An example alert message!', 'notice_until': '0x24bcca57c00', 'signatures': ['0xbd07059aa9a3d057da294c2c4d96fa1e67eeb089837c87b523f124239e18e9fc7d11bb95b720478f7f937d073517d0e4eb9a91d12da5c88a05f750362f4c214dd0', '0x0242ef40bb64fe3189284de91f981b17f4d740c5e24a3fc9b70059db6aa1d198a2e76da4f84ab37549880d116860976e0cf81cd039563c452412076ebffa2e4453']}], 'id': 42}
//  sdk post':{'id': 0, 'jsonrpc': '2.0', 'method': 'send_alert', 'params': [{'cancel': '0x0', 'id': '0x1', 'max_version': None, 'message': 'An example alert message!', 'min_version': None, 'notice_until': '0x24bcca57c00', 'priority': '0x1', 'signatures': ['0xbd07059aa9a3d057da294c2c4d96fa1e67eeb089837c87b523f124239e18e9fc7d11bb95b720478f7f937d073517d0e4eb9a91d12da5c88a05f750362f4c214dd0', '0x0242ef40bb64fe3189284de91f981b17f4d740c5e24a3fc9b70059db6aa1d198a2e76da4f84ab37549880d116860976e0cf81cd039563c452412076ebffa2e4453']}]}
#[rstest(mock_rpc_data("send_alert", "AlertFailedToVerifySignatures"))]
#[ignore]
fn send_alert_alert_failed_to_verify_signatures(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();
    let alert = serde_json::from_value(mock_rpc_data.request_data["params"][0].clone()).unwrap();

    let _response = ckb_client.send_alert(alert).unwrap();
}
