use rstest::rstest;
use crate::mockrpc::{mock_rpc_data, MockRpcData};


#[rstest(mock_rpc_data("get_banned_addresses", "[]"))]
fn get_banned_addresses(mock_rpc_data: MockRpcData) {
    let ckb_client = mock_rpc_data.client();

    let response = ckb_client.get_banned_addresses().unwrap();
    let response_data = &mock_rpc_data.response_data["result"].as_array().unwrap();

    assert_eq!(response_data.len(), response.len());

    for i in 0..response_data.len() {
        let expected = &response_data[i];
        let actual = &response[i];

        assert_eq!(expected["address"], actual.address);
        assert_eq!(expected["ban_reason"], actual.ban_reason);
        assert_eq!(expected["ban_until"], actual.ban_until.to_string());
        assert_eq!(expected["created_at"], actual.created_at.to_string());
    }
}
